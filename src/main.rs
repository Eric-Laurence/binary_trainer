use eframe::egui;
use rand::Rng;
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose};

// debug to be able to print it
// clone to be able to copy it
#[derive(Debug, Clone, PartialEq)]
enum NumberBase {
    Binary,
    Decimal,
    Hexadecimal,
    Base64,
}

struct NumberConverterApp {
    range_input: String,           // string of possible numbers to generate
    input_base: NumberBase,        // input base
    output_base: NumberBase,       // output base

    pad_input: bool,
    pad_output: bool,
    input_length: u32,             // # of bits
    output_length: u32,
    
    generated_number: Option<u64>, // current number
    show_converted: bool,
    error_message: String,         // to display errors
}

// give default values
impl Default for NumberConverterApp {
    fn default() -> Self {
        Self {
            range_input: String::new(),
            input_base: NumberBase::Decimal,
            output_base: NumberBase::Binary,

            pad_input: false,
            pad_output: false,
            input_length: 8,
            output_length: 8,

            generated_number: None,
            show_converted: false,
            error_message: String::new(),
        }
    }
}



impl NumberConverterApp {
    // parse inputs for all possible numbers to generate
    fn parse_range(range_str: &str) -> Result<Vec<u64>, String> {
        // remove whitespace and split by comma
        let parts: Vec<&str> = range_str.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        
        let mut numbers = Vec::new();
        
        for part in parts {
            if part.contains('-') {
                // convert ranges into individual values
                let range_parts: Vec<&str> = part.split('-').collect();
                if range_parts.len() != 2 {
                    return Err(format!("Invalid range format: {}", part));
                }
                
                let start = u64::from_str(range_parts[0].trim())
                    .map_err(|_| format!("Invalid number: {}", range_parts[0]))?;
                let end = u64::from_str(range_parts[1].trim())
                    .map_err(|_| format!("Invalid number: {}", range_parts[1]))?;
                
                if start > end {
                    return Err(format!("Invalid range: {} > {}", start, end));
                }
                
                // add all the numbers in the range
                for n in start..=end {
                    numbers.push(n);
                }
            }
            else {
                // not a range, add the number
                let num = u64::from_str(part)
                    .map_err(|_| format!("Invalid number: {}", part))?;
                numbers.push(num);
            }
        }
        
        Ok(numbers)
    }
    
    // convert number to string (in specified base)
    fn number_to_string(num: u64, base: &NumberBase, pad: bool, pad_length: u32) -> String {
        match base {
            // {:#b} and {:#x} rather than {:b} and {:x} show 0b... and 0x...
            NumberBase::Binary => {
                let s = format!("{:b}", num);
                if pad {
                    format!("{:0>width$}", s, width = pad_length as usize)
                }
                else {
                    s
                }
            },
            NumberBase::Decimal => num.to_string(),
            NumberBase::Hexadecimal => {
                let s = format!("{:x}", num);
                if pad {
                    format!("{:0>width$}", s, width = pad_length as usize)
                }
                else {
                    s
                }
            },
            NumberBase::Base64 => {
                // convert number to bytes and then to base64
                
                // method 1: convert all 8 bytes to base64
                // let bytes = num.to_be_bytes();

                // method 2: convert minimum bytes needed
                // easier to understand and prevents clutter
                let bytes = if num == 0 {
                    vec![0]
                }
                else {
                    num.to_be_bytes()[num.to_be_bytes().len() - (num.ilog(256) as usize + 1)..].to_vec()
                };

                general_purpose::STANDARD.encode(bytes)
            }
        }
    }
    
    // generate a random number from the set of possible values
    fn generate_random(&mut self) {
        match Self::parse_range(&self.range_input) {
            Ok(numbers) if !numbers.is_empty() => {
                // pick a random index
                let idx = rand::thread_rng().gen_range(0..numbers.len());
                self.generated_number = Some(numbers[idx]);
                self.error_message.clear();
            }
            Ok(_) => {
                self.error_message = "No valid numbers provided".to_string();
            }
            Err(e) => {
                self.error_message = e;
            }
        }
    }
}



impl eframe::App for NumberConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Number Base Converter");
            
            ui.horizontal(|ui| {
                // input base dropdown
                ui.label("Input Base:");
                egui::ComboBox::from_label("input")
                    .selected_text(format!("{:?}", self.input_base))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.input_base, NumberBase::Binary, "Binary");
                        ui.selectable_value(&mut self.input_base, NumberBase::Decimal, "Decimal");
                        ui.selectable_value(&mut self.input_base, NumberBase::Hexadecimal, "Hexadecimal");
                        ui.selectable_value(&mut self.input_base, NumberBase::Base64, "Base64");
                    });
                
                // output base dropdown
                ui.label("Output Base:");
                egui::ComboBox::from_label("output")
                    .selected_text(format!("{:?}", self.output_base))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.output_base, NumberBase::Binary, "Binary");
                        ui.selectable_value(&mut self.output_base, NumberBase::Decimal, "Decimal");
                        ui.selectable_value(&mut self.output_base, NumberBase::Hexadecimal, "Hexadecimal");
                        ui.selectable_value(&mut self.output_base, NumberBase::Base64, "Base64");
                    });
            });
            
            ui.add_space(10.0);

            ui.vertical(|ui| {
                ui.checkbox(&mut self.pad_input, "Pad input");
                if self.pad_input {
                    ui.add(egui::Slider::new(&mut self.input_length, 1..=64).text("bits"));
                }

                ui.checkbox(&mut self.pad_output, "Pad output");
                if self.pad_output {
                    ui.add(egui::Slider::new(&mut self.output_length, 1..=64).text("bits"));
                }
            });

            ui.add_space(10.0);
            
            // set of values to generate from
            ui.label("Enter numbers or ranges (e.g., '1-5, 7, 10-15'):");
            ui.text_edit_multiline(&mut self.range_input);
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                if ui.button("Generate").clicked() {
                    self.generate_random();
                    self.show_converted = false;
                }
                if ui.button("Check").clicked() {
                    // conversion is done via number to string
                    self.show_converted = true;
                }
            });
            
            ui.add_space(1.0);
            
            // display generated value
            if let Some(num) = self.generated_number {
                ui.vertical(|ui| {
                    // in input base
                    ui.add(egui::Label::new(
                        egui::RichText::new(Self::number_to_string(num, &self.input_base, self.pad_input, self.input_length))
                            .size(32.0)
                    ));

                    // in output base
                    if self.show_converted {
                        ui.add_space(5.0);

                        ui.add(egui::Label::new(
                            egui::RichText::new(Self::number_to_string(num, &self.output_base, self.pad_output, self.output_length))
                                .size(32.0)
                        ));
                    }
                });
            }


            
            
            // display error messages for debugging
            if !self.error_message.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.error_message);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // set default window size
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 500.0)),
        ..Default::default()
    };
    
    // open the window
    eframe::run_native(
        "Number Base Converter",
        options,
        Box::new(|_cc| Box::new(NumberConverterApp::default())),
    )
}