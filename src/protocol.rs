use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn write<W: AsyncWrite + Unpin>(writer: &mut W, data: &[u8]) -> Result<()> {
    let len = data.len();
    writer.write_u64(len as u64).await?;
    writer.write_all(data).await?;
    Ok(())
}

pub async fn read<W: AsyncRead + Unpin>(reader: &mut W, buf: &mut Vec<u8>) -> Result<()> {
    let len = reader.read_u64().await?;
    let mut reader = reader.take(len);
    reader.read_to_end(buf).await?;
    Ok(())
}
