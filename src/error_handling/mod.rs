mod box_example;
use box_example::box_example;

mod custom_error_type;
use custom_error_type::custom_error_type_example;

mod thiserror_example;
use thiserror_example::thiserror_example;

mod anyhow_example;
use anyhow_example::anyhow_example;

pub fn error_handling_examples() {
    box_example().map_err(|err| println!("{:?}", err)).ok();

    custom_error_type_example()
        .map_err(|err| println!("{:?}", err))
        .ok();

    thiserror_example()
        .map_err(|err| println!("{:?}", err))
        .ok();

    anyhow_example().map_err(|err| println!("{:?}", err)).ok();
}
