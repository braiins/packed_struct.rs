use prelude::v1::*;

pub struct DebugBinaryByteSlice<'a> {
    pub bits: &'a Range<usize>,
    pub slice: &'a [u8]
}

impl<'a> fmt::Binary for DebugBinaryByteSlice<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        for i in self.bits.start..self.bits.end {
            let byte = i / 8;
            let bit = i % 8;
            let bit = 7 - bit;

            let src_byte = self.slice[byte];
            let src_bit = (src_byte & (1 << bit)) == (1 << bit);

            let s = if src_bit { "1" } else { "0" };
            try!(fmt.write_str(s));
        }

        Ok(())
    }
}

pub struct DebugBitField<'a> { 
	pub name: Cow<'a, str>,
	pub bits: Range<usize>,
	pub display_value: Cow<'a, str>
}


pub fn packable_fmt_fields(f: &mut Formatter, packed_bytes: &[u8], fields: &[DebugBitField]) -> fmt::Result {
    if fields.len() == 0 {
		return Ok(());
	}

    let max_field_length_name = fields.iter().map(|x| x.name.len()).max().unwrap();
	let max_bit_width = fields.iter().map(|x| x.bits.len()).max().unwrap();

    if max_bit_width > 32 {
        for field in fields {
            try!(write!(f, "{name:>0$} | {base_value:?}\r\n",
                            max_field_length_name + 1,
                            base_value = field.display_value,
                            name = field.name
                            ));
        }
    } else {    
        for field in fields {

            let debug_binary = DebugBinaryByteSlice {
                bits: &field.bits,
                slice: packed_bytes
            };

            try!(write!(f, "{name:>0$} | bits {bits_start:>3}..{bits_end:<3} | 0b{binary_value:>0width_bits$b}{dummy:>0spaces$} | {base_value:?}\r\n",
                            max_field_length_name + 1,
                            base_value = field.display_value,
                            binary_value = debug_binary,
                            dummy = "",
                            bits_start = field.bits.start,
                            bits_end = field.bits.end,
                            width_bits = field.bits.len(),
                            spaces = (max_bit_width - field.bits.len()) as usize,
                            name = field.name
                            ));
        }
    }

    Ok(())
}

/*
pub fn packable_fmt_bits(f: &mut Formatter, fields: &[DebugBitField]) -> fmt::Result {
	if fields.len() == 0 {
		return Ok(());
	}

	let max_field_length_name = fields.iter().map(|x| x.name.len()).max().unwrap();
	let max_bit_width = fields.iter().map(|x| x.bits.len()).max().unwrap();

    if max_bit_width > 8 {
        return Ok(());
    }
	
    try!(write!(f, "| "));

    let mut prev: Option<&DebugBitField> = None;

    for field in fields {
        if let Some(prev) = prev {
            // middle
            let empty_bits = field.bits.start - prev.bits.end;
            if empty_bits > 0 {
                print_empty_bits(f, empty_bits);
                try!(write!(f, " | "));
            }
        } else {
            // beginning
            let empty_bits = field.bits.start;
            if empty_bits > 0 {
                print_empty_bits(f, empty_bits);
                try!(write!(f, " | "));
            }
        }

        try!(write!(f, "{value:>0width_bits$b}", value = field.value, width_bits = field.bits.len()));

        try!(write!(f, " | "));

        prev = Some(field);
    }

    // end
    if let Some(prev) = prev {
        if prev.bits.end != max_bit_width {
            let empty_bits = max_bit_width as isize - prev.bits.end as isize;
            if empty_bits > 0 {
                print_empty_bits(f, empty_bits as usize);
                try!(write!(f, " |"));
            }
        }
    }

    Ok(())
}
*/