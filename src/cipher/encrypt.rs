use crate::cipher::Cipher;
use anyhow::Result;
use std::io::{Bytes, Read, Write};
use std::iter::Fuse;

pub struct Encrypter<W, C, const N: usize> {
    writer: W,
    cipher: C,
}

impl<W: Write, C: Cipher<N>, const N: usize> Encrypter<W, C, N> {
    pub fn new(writer: W, cipher: C) -> Self {
        Encrypter { writer, cipher }
    }

    pub fn encrypt<R>(&mut self, reader: R) -> Result<()>
    where
        R: Read,
    {
        for byte_pair in BytePairs::new(reader) {
            let byte_pair = byte_pair?;
            let encrypted = self.cipher.encrypt_char_pair(byte_pair.0, byte_pair.1);
            self.writer.write_all(&encrypted)?;
        }
        Ok(())
    }
}

struct BytePairs<R> {
    bytes: Fuse<Bytes<R>>,
}

impl<R: Read> BytePairs<R> {
    fn new(reader: R) -> Self {
        BytePairs {
            // N.B. Bytes can theoretically return a Some after having returned a None.
            // This is a general property of iterators, but is of specific concern for Bytes.
            // The underlying reader may return 0 on a read call,
            // but then start returning data again.
            // This would screw up our pair logic, which can't handle when the first
            // byte is None, but the second byte is Some.
            // (a BytePair always needs a first byte, only the second byte it optional).
            //
            // We use a Fuse so that as soon as we see a None from the Bytes iterator,
            // we consider ourself to be done.
            bytes: reader.bytes().fuse(),
        }
    }

    fn next_pair(&mut self) -> Result<Option<BytePair>> {
        let pair = match (self.bytes.next(), self.bytes.next()) {
            (Some(b0), Some(b1)) => Some((b0?, Some(b1?))),
            (Some(b0), None) => Some((b0?, None)),
            _ => None,
        };
        Ok(pair)
    }
}

impl<R: Read> Iterator for BytePairs<R> {
    type Item = Result<BytePair>;

    fn next(&mut self) -> Option<Result<BytePair>> {
        self.next_pair().transpose()
    }
}

type BytePair = (u8, Option<u8>);
