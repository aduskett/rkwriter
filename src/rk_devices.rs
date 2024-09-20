
pub mod rk_usb_type {
    pub const MASK_ROM: u16 = 0x01;
    pub const LOADER: u16 = 0x02;
    //pub const MSC: u8 = 0x04;
}

mod rk_usb_info {
    pub const USB_VENDOR_ID: u16 = 0x2207;
}

/// Holds information from libusb for each found Rockchip device.
/// - bus_number: The number of the bus that the Rockchip device is connected to.
/// - vendor_id: The Rockchip devices vendor ID.
/// - product_id: The Rockchip devices product ID.
/// - location: Derived from the USB specification release number. LOADER if in recovery mode.
pub struct RKDevice {
    pub bus_number: u8,
    pub vendor_id: u16,
    pub product_id: u16,
    pub location: u16,
    pub location_string: String,
}

/// Stores a vector of found devices. If any devices are found, found is set to true.
pub struct RKDevices {
    pub devices: Vec<RKDevice>,
    pub found: bool,
}

/// Rockchip devices. Currently used only to search for connected USB devices.
impl RKDevices {
    pub fn new() -> Self {
        RKDevices {
            devices: Vec::new(),
            found: false,
        }
    }

    /// Get the type of USB from a connected Rockchip device.
    /// If a device is in recovery mode, "Loader" should be returned.
    /// If a device is fully loaded, MASK_ROM should be returned.
    /// Otherwise, return UNKNOWN.
    fn get_usb_type(&mut self, usb_version_raw: &str) -> u16 {
        let usb_version = usb_version_raw.replace(".", "");
        let usb_version_int = usb_version.parse::<u16>().unwrap();
        let temp = usb_version_int & 0x1;
        if temp == 0 {
            return rk_usb_type::MASK_ROM;
        }
        rk_usb_type::LOADER
    }

    /// Check if a connected USB device is a Rockchip device.
    fn check_if_rockchip(&mut self, bus_number: u8, vendor_id: u16, product_id: u16, usb_version: &str) -> bool {
        if vendor_id == rk_usb_info::USB_VENDOR_ID && product_id >> 8 > 0 {
            let location = self.get_usb_type(&usb_version);
            let location_string = (match location {
                rk_usb_type::MASK_ROM => "Maskrom",
                rk_usb_type::LOADER => "Loader",
                _ => "Unknown",
            }).to_string();
            let device = RKDevice {
                bus_number,
                vendor_id,
                product_id,
                location,
                location_string
            };
            self.devices.push(device);
            self.found = true;
            return true
        }
        false
    }

    /// Iterate through all connected USB devices and check if it's a Rockchip device.
    /// If so, the device is stored in the devices vector.
    pub fn search(&mut self) {
        for device in rusb::devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            let usb_version = device_desc.usb_version().to_string();
            self.check_if_rockchip(device.bus_number(), device_desc.vendor_id(), device_desc.product_id(), &usb_version);
        };
    }

    /// List all connected Rockchip devices in a human friendly format.
    pub fn list_devices(&mut self) {
        self.search();
        if self.found {
            let num_devices = self.devices.len();
            println!("Found {} connected Rockchip USB {}",
                     num_devices,
                     if num_devices == 1 { "device" } else { "devices" }
            );
            for device in &self.devices {
                println!("BusNum={} VID={:#02x} PID={:#02x} Mode={}",
                         device.bus_number,
                         device.vendor_id,
                         device.product_id,
                         device.location_string)
            }
        } else {
            println!("No devices found!");
        }
    }
}
