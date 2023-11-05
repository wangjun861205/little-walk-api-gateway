use crate::core::error::Error;
use bytes::{BufMut, Bytes, BytesMut};
use futures::{stream, Stream, StreamExt, TryStreamExt};

pub async fn stream_to_bytes(
    stream: impl Stream<Item = Result<Bytes, Error>>,
) -> Result<Bytes, Error> {
    let bs: BytesMut = stream
        .try_collect::<Vec<Bytes>>()
        .await?
        .into_iter()
        .fold(BytesMut::new(), |mut s, v| {
            s.put(v);
            s
        });
    Ok(bs.freeze())
}
