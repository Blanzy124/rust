use reqwest::Error;
use sqlx::query_as_unchecked;


pub struct YarnAddition {
    lot_number: String,
    size: String,
    color: String,
    available: bool,
    content: Option<String>,
    location: String,
    note: Option<String>
}

impl YarnAddition {
    pub fn new (lot_number: String, size: String, color: String, content: String, location: String, note: String) 
    -> Result<Self, String>{
        
        if lot_number.is_empty() || size.is_empty() || !color.is_empty() || location.is_empty(){
            return Err("Lot number, size, color or location are missing".into());
        }

        if !lot_number.is_empty() && !size.is_empty() && !color.is_empty() && !location.is_empty() && content.is_empty() && note.is_empty() {
            return Ok(Self { lot_number, size, color, available: true, content: (None), location, note: (None) });
        } 
        if !lot_number.is_empty() && !size.is_empty() && !color.is_empty() && !location.is_empty() && !content.is_empty() && note.is_empty() {
            return Ok(Self { lot_number, size, color, available: true, content: content.into(), location, note: (None) });
        } 
        if !lot_number.is_empty() && !size.is_empty() && !color.is_empty() && !location.is_empty() && content.is_empty() && !note.is_empty() {
            return Ok(Self { lot_number, size, color, available: true, content: (None), location, note: note.into() });
        } 
        else {
            return Err("Some error on the yarn addition".into());
        }
        }
}