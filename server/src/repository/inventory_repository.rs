use crate::repository::model::item_model::InventoryItemModel;
use crate::repository::persistence_error::PersistenceError;
use crate::repository::{InventoryRepository, PgRepository};
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, Executor, Row};

use crate::server::model::events::game_event::CharacterRemoveItem;
use async_trait::async_trait;

#[async_trait]
impl InventoryRepository for PgRepository {
    async fn character_inventory_update_add(&self, inventory_update_items: &[InventoryItemUpdate], buy: bool) -> Result<(), Error> {
        if inventory_update_items.is_empty() {
            return Ok(());
        }
        let stackable_items = inventory_update_items.iter().filter(|item| item.stackable).collect::<Vec<&InventoryItemUpdate>>();
        let not_stackable_items = inventory_update_items.iter().filter(|item| !item.stackable).collect::<Vec<&InventoryItemUpdate>>();
        let mut tx = self.pool.begin().await.unwrap();
        let updated_item_ids_amounts: Vec<(i32, i16)> = inventory_update_items.iter().map(|item| (item.item_id, item.amount)).collect();
        tx.execute(sqlx::query("INSERT INTO inventory (char_id, nameid, amount, identified) \
        (SELECT * FROM UNNEST($1::int4[], $2::int4[], $3::int2[], $4::bool[])) \
        ON CONFLICT (char_id, nameid, unique_id)\
         DO UPDATE set amount = inventory.amount + EXCLUDED.amount")
            .bind(stackable_items.iter().map(|i| i.char_id).collect::<Vec<i32>>())
            .bind(stackable_items.iter().map(|i| i.item_id).collect::<Vec<i32>>())
            .bind(stackable_items.iter().map(|i| i.amount).collect::<Vec<i16>>())
            .bind(stackable_items.iter().map(|i| i.identified).collect::<Vec<bool>>())).await?;
        tx.execute(sqlx::query("INSERT INTO inventory (char_id, nameid, amount, identified, unique_id) \
        (SELECT * FROM UNNEST($1::int4[], $2::int4[], $3::int2[], $4::bool[], $5::int8[]))")
            .bind(not_stackable_items.iter().map(|i| i.char_id).collect::<Vec<i32>>())
            .bind(not_stackable_items.iter().map(|i| i.item_id).collect::<Vec<i32>>())
            .bind(not_stackable_items.iter().map(|i| i.amount).collect::<Vec<i16>>())
            .bind(not_stackable_items.iter().map(|i| i.identified).collect::<Vec<bool>>())
            .bind(not_stackable_items.iter().map(|i| i.unique_id).collect::<Vec<i64>>())
        ).await?;
        if buy {
            let item_ids_prices = tx.fetch_all(sqlx::query("SELECT DISTINCT id, price_buy FROM item_db WHERE id IN (SELECT * FROM UNNEST($1::int4[]))")
                .bind(updated_item_ids_amounts.iter().map(|(id, _amount)| *id).collect::<Vec<i32>>())).await?;
            let cost = updated_item_ids_amounts.iter().fold(0, |mut acc, (id, amount)| {
                let price = item_ids_prices.iter().find(|item_price| item_price.get::<i32, _>(0) == *id).map_or(0, |item_price| item_price.get::<i32, _>(1));
                acc += (*amount as i32) * price;
                acc
            });
            let updated_zeny = tx.fetch_all(sqlx::query("UPDATE char set zeny = zeny - $1 WHERE char_id = $2 RETURNING zeny;")
                .bind(cost)
                .bind(inventory_update_items[0].char_id)
            ).await?;
            let zeny: i32 = updated_zeny[0].get(0);
            if zeny >= 0 {
                tx.commit().await
            } else {
                tx.rollback().await?;
                Err(PersistenceError::new("Rollbacking buy: not enough zeny".to_string()).into())
            }
        } else {
            tx.commit().await
        }
    }

    async fn character_inventory_update_remove(&self, inventory_update_items: &Vec<(InventoryItemModel, CharacterRemoveItem)>, sell: bool) -> Result<(), Error> {
        if inventory_update_items.is_empty() {
            return Ok(());
        }
        let item_to_delete = inventory_update_items.iter().filter(|(item,_)| item.amount <= 0).map(|(item, _)| item.id).collect::<Vec<i32>>();
        let item_to_update = inventory_update_items.iter().filter(|(item,_)| item.amount > 0).map(|(item, _)| (item.id, item.amount)).collect::<Vec<(i32, i16)>>();

        let mut tx = self.pool.begin().await.unwrap();
        tx.execute(sqlx::query("UPDATE inventory as inv SET amount = new.amount FROM (select unnest($1::int4[]) as id,unnest($2::int2[]) as amount) as new WHERE inv.id = new.id ")
            .bind(item_to_update.iter().map(|(id,_)| *id).collect::<Vec<i32>>())
            .bind(item_to_update.iter().map(|(_, amount)| *amount).collect::<Vec<i16>>())
        ).await?;
        tx.execute(sqlx::query("DELETE FROM inventory as inv WHERE inv.id IN (SELECT * FROM UNNEST($1::int4[])) and inv.equip = 0").bind(item_to_delete)).await?;
        if sell {
            let mut zeny = 0;
            inventory_update_items.iter().for_each(|(_, item)| zeny += item.amount as i32 * item.price);
            let _updated_zeny = tx.fetch_all(sqlx::query("UPDATE char set zeny = zeny + $1 WHERE char_id = $2 RETURNING zeny;")
                .bind(zeny)
                .bind(inventory_update_items[0].1.char_id as i32)
            ).await?;
        }
        tx.commit().await
    }


    async fn character_inventory_delete(&self, delete_items: DeleteItems) -> Result<PgQueryResult, Error> {
        if delete_items.amount > 0 && delete_items.unique_id == 0 {
            sqlx::query("UPDATE inventory SET amount = $1 WHERE char_id = $2 AND id = $3 ")
                .bind(delete_items.amount)
                .bind(delete_items.char_id)
                .bind(delete_items.item_inventory_id)
            .execute(&self.pool).await
        } else {
            sqlx::query("DELETE FROM inventory WHERE char_id = $1 AND id = $2")
                .bind(delete_items.char_id)
                .bind(delete_items.item_inventory_id)
                .bind(delete_items.unique_id)
               .execute(&self.pool).await
        }
    }

    async fn character_inventory_fetch(&self, char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
        sqlx::query_as("SELECT inv.id, inv.unique_id, inv.nameid, inv.amount, inv.damaged, inv.refine, inv.identified, inv.equip, item.name_english, item.type, item.weight, inv.card0, inv.card1, inv.card2, inv.card3
                            FROM inventory inv JOIN item_db item ON inv.nameid = item.id where inv.char_id = $1")
            .bind(char_id)
            .fetch_all(&self.pool).await
    }

    async fn character_inventory_wearable_item_update(&self, items: Vec<InventoryItemModel>) -> Result<PgQueryResult, Error> {
        sqlx::query("UPDATE inventory as inv SET equip = new.equip FROM (select unnest($1::int4[]) as id,unnest($2::int4[]) as equip) as new WHERE inv.id = new.id")
            .bind(items.iter().map(|i| i.id).collect::<Vec<i32>>())
            .bind(items.iter().map(|i| i.equip).collect::<Vec<i32>>())
            .execute(&self.pool).await
    }
}