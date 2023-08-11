
pub trait TuningUpdatedListenerInterface: TuningUpdated {}

pub trait TuningUpdated {

    fn tuning_updated(&mut self, new_tuning: &Tuning);
}

pub trait NotesOnChangedListenerInterface: NoteOn + NoteOff {}

pub trait NoteOn {
    
    fn note_on(&mut self, note_num: i32);
}

pub trait NoteOff {

    
    fn note_off(&mut self, note_num: i32);
}


