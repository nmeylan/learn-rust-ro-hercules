#[macro_export]
macro_rules! read_lock {
    ( $rw_lock:expr ) => {
    $rw_lock.read().unwrap()
  };
}
#[macro_export]
macro_rules! write_lock {
    ( $rw_lock:expr ) => {
    $rw_lock.write().unwrap()
  };
}

#[macro_export]
macro_rules! read_session {
    ( $session_map:expr, $session_id:expr ) => {
        crate::read_lock!($session_map.get(&$session_id).unwrap())
  };
}
#[macro_export]
macro_rules! write_session {
    ( $session_map:expr, $session_id:expr ) => {
        crate::write_lock!($session_map.get(&$session_id).unwrap())
  };
}
#[macro_export]
macro_rules! cast {
    ( $packet:expr, $type:ty ) => {
       $packet.as_any().downcast_ref::<$type>().unwrap();
  };
}

#[macro_export]
macro_rules! socket_send {
    ( $tcp_stream:expr, $data:expr ) => {
    {
       let mut tcp_stream_guard = $tcp_stream.write().unwrap();
        tcp_stream_guard.write($data);
        tcp_stream_guard.flush();
    }
  }
}