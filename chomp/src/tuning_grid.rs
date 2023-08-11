crate::ix!();

/**
  | An auto-generated component, created
  | by the Projucer.
  | 
  | Describe your class and how it works
  | here!
  |
  */
pub struct TWSTuningGrid {
    mod_:  Box<TuningTableListBoxModel>,
    table: Box<TableListBox>,
}

impl Component              for TWSTuningGrid {}
impl TuningUpdatedListener  for TWSTuningGrid {}
impl NotesOnChangedListener for TWSTuningGrid {}

impl TWSTuningGrid {

    pub fn tuning_updated(&mut self, new_tuning: &Tuning)  {
        
        todo!();
        /*
            mod->tuningUpdated(newTuning);
        */
    }
    
    pub fn note_on(&mut self, note_num: i32)  {
        
        todo!();
        /*
            mod->noteOn(noteNum);
        */
    }
    
    pub fn note_off(&mut self, note_num: i32)  {
        
        todo!();
        /*
            mod->noteOff(noteNum);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
impl TWSTuningGrid {
    
    pub fn new() -> Self {
    
        todo!();
        /*
        //[Constructor_pre] You can add your own custom stuff here..
        //[/Constructor_pre]

        table.reset (new TableListBox());
        addAndMakeVisible (table.get());
        table->setName ("new component");


        //[UserPreSize]
        //[/UserPreSize]

        setSize (300, 300);


        //[Constructor] You can add your own custom stuff here..
        mod = std::make_unique<surgesynthteam::TuningTableListBoxModel>();
        mod->setTableListBox( table.get() );
        mod->setupDefaultHeaders( table.get() );

        table->setModel( mod.get() );

        table->getViewport()->setScrollBarsShown(true,false);
        table->getViewport()->setViewPositionProportionately( 0.0,  60.0 / 127.0 );
        //[/Constructor]
        */
    }
}

impl Drop for TWSTuningGrid {

    fn drop(&mut self) {
        todo!();
        /*
            //[Destructor_pre]. You can add your own custom destruction code here..
        //[/Destructor_pre]

        table = nullptr;


        //[Destructor]. You can add your own custom destruction code here..
        //[/Destructor]
        */
    }
}

impl TWSTuningGrid {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            //[UserPrePaint] Add your own custom painting code here..
        //[/UserPrePaint]

        g.fillAll (Colour (0xff323e44));

        //[UserPaint] Add your own custom painting code here..
        //[/UserPaint]
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            //[UserPreResize] Add your own custom resize code here..
        //[/UserPreResize]

        table->setBounds (0, 0, proportionOfWidth (1.0000f), proportionOfHeight (1.0000f));
        //[UserResized] Add your own custom resize handling here..
        table->getViewport()->setScrollBarsShown(true,false);
        table->getViewport()->setViewPositionProportionately( 0.0, 60.0 / 127.0 );
        //[/UserResized]
        */
    }
}
