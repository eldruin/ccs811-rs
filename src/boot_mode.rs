use hal::digital::v2::OutputPin;
use {
    hal, mode, Ccs811, Ccs811Awake, Ccs811BootMode, Ccs811Device, Error, ErrorAwake,
    ModeChangeError, Register,
};

impl<I2C, E> Ccs811BootMode for Ccs811Awake<I2C, mode::Boot>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    type ModeChangeError = ModeChangeError<ErrorAwake<E>, Self>;
    type AppModeType = Ccs811Awake<I2C, mode::App>;

    fn app_start(mut self) -> Result<Self::AppModeType, Self::ModeChangeError> {
        match self.has_valid_app() {
            Err(e) => Err(ModeChangeError::new(self, e)),
            Ok(is_valid) => {
                if !is_valid {
                    Err(ModeChangeError::new(self, ErrorAwake::NoValidApp))
                } else {
                    match self.write_register_no_data(Register::APP_START) {
                        Err(e) => Err(ModeChangeError::new(self, e)),
                        Ok(_) => Ok(Ccs811Awake::create(self.i2c, self.address)),
                    }
                }
            }
        }
    }
}

impl<I2C, CommE, PinE, NWAKE> Ccs811BootMode for Ccs811<I2C, NWAKE, mode::Boot>
where
    I2C: hal::blocking::i2c::Write<Error = CommE> + hal::blocking::i2c::WriteRead<Error = CommE>,
    NWAKE: OutputPin<Error = PinE>,
{
    type ModeChangeError = ModeChangeError<Error<CommE, PinE>, Self>;
    type AppModeType = Ccs811<I2C, NWAKE, mode::App>;

    fn app_start(mut self) -> Result<Self::AppModeType, Self::ModeChangeError> {
        if let Err(e) = self.n_wake_pin.set_low() {
            return Err(ModeChangeError::new(self, Error::Pin(e)));
        }
        let Ccs811 {
            dev,
            mut n_wake_pin,
            _mode,
        } = self;
        let result = dev.app_start();
        if let Err(e) = n_wake_pin.set_high() {
            return match result {
                Ok(Ccs811Awake {
                    i2c,
                    address,
                    _mode,
                }) => Err(ModeChangeError {
                    dev: Ccs811::create(i2c, n_wake_pin, address),
                    error: Error::Pin(e),
                }),
                Err(ModeChangeError { dev, error }) => Err(ModeChangeError {
                    dev: Ccs811::from_awake_dev(dev, n_wake_pin),
                    error: error.into(),
                }),
            };
        }
        match result {
            Ok(dev) => Ok(Ccs811::from_awake_dev(dev, n_wake_pin)),
            Err(ModeChangeError { dev, error }) => Err(ModeChangeError {
                dev: Ccs811::from_awake_dev(dev, n_wake_pin),
                error: error.into(),
            }),
        }
    }
}
