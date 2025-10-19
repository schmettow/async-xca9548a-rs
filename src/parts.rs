use crate::{DoOnAcquired, Error, SelectChannels};
use core::marker::PhantomData;
use embedded_hal_async::i2c as ehal;

/// Slave I2C device
pub struct I2cSlave<'a, DEV: 'a, I2C>(&'a DEV, u8, PhantomData<I2C>);

macro_rules! parts {
    ( $name:ident; $( $i2cx:ident, $channel:expr ),+ ) => {

        /// Slave I2C devices
        pub struct $name<'a, DEV:'a, I2C> {
            $(
                /// Slave I2C device
                pub $i2cx: I2cSlave<'a, DEV, I2C>,
            )*
        }

        impl<'a, DEV:'a, I2C> $name<'a, DEV, I2C> {
            pub(crate) fn new(dev: &'a DEV) -> Self {
                $name {
                    $(
                        $i2cx: I2cSlave(&dev, $channel, PhantomData),
                    )*
                }
            }
        }
    }
}
parts!(
    Parts; i2c0, 0x01, i2c1, 0x02, i2c2, 0x04, i2c3, 0x08, i2c4, 0x10, i2c5, 0x20, i2c6, 0x40, i2c7, 0x80
);
parts!(
    Parts2; i2c0, 0x01, i2c1, 0x02
);
parts!(
    Parts4; i2c0, 0x01, i2c1, 0x02, i2c2, 0x04, i2c3, 0x08
);

impl<'a, DEV, I2C, E> ehal::ErrorType for I2cSlave<'a, DEV, I2C>
where
    DEV: DoOnAcquired<'a, I2C>,
    I2C: ehal::I2c<Error = E>,
    E: ehal::Error,
{
    type Error = Error<E>;
}

impl<'a, DEV, I2C, E> ehal::I2c for I2cSlave<'a, DEV, I2C>
where
    DEV: DoOnAcquired<'a, I2C>,
    I2C: ehal::I2c<Error = E>,
    E: ehal::Error,
{
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [ehal::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.0
            .do_on_acquired(async |mut dev| {
                if dev.selected_channel_mask != self.1 {
                    dev.select_channels(self.1).await?;
                }
                dev.i2c
                    .transaction(address, operations)
                    .await
                    .map_err(Error::I2C)
            })
            .await
    }

    async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.0
            .do_on_acquired(async |mut dev| {
                if dev.selected_channel_mask != self.1 {
                    dev.select_channels(self.1).await?;
                }
                dev.i2c.read(address, read).await.map_err(Error::I2C)
            })
            .await
    }

    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.0
            .do_on_acquired(async |mut dev| {
                if dev.selected_channel_mask != self.1 {
                    dev.select_channels(self.1).await?;
                }
                dev.i2c.write(address, write).await.map_err(Error::I2C)
            })
            .await
    }

    async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.0
            .do_on_acquired(async |mut dev| {
                if dev.selected_channel_mask != self.1 {
                    dev.select_channels(self.1).await?;
                }
                dev.i2c
                    .write_read(address, write, read)
                    .await
                    .map_err(Error::I2C)
            })
            .await
    }
}
