use std::fs::File;
use std::io::Result;

use csv::Writer;
use crate::features::window::WindowFeatures;

pub fn write_csv(path: &str, 
    window: &[WindowFeatures],) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = Writer::from_writer(file);
        
        for window in windows {
            writer.serialize(window)?;
        }
        
        writer.flush()?;
        Ok(())
    }