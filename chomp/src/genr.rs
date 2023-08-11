crate::ix!();

/**
  | An auto-generated component, created
  | by the Projucer.
  | 
  | Describe your class and how it works
  | here!
  |
  */
pub struct TWSKBMGenerator {
    processor:       &mut TuningworkbenchsynthAudioProcessor,
    group_component: Box<GroupComponent>,
    scale_start:     Box<TextEditor>,
    tune_note:       Box<TextEditor>,
    tune_freq:       Box<TextEditor>,
    apply_button:    Box<TextButton>,
}

impl Component for TWSKBMGenerator { }
impl ButtonListener for TWSKBMGenerator { }

impl TWSKBMGenerator {
    
    pub fn new(p: &mut TuningworkbenchsynthAudioProcessor) -> Self {
    
        todo!();
        /*


        
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
   
    pub fn button_clicked(&mut self, button_that_was_clicked: *mut Button)  {
        
        todo!();
        /*
        
        */
    }
}

impl TWSKBMGenerator {

    pub fn new(p: &mut TuningworkbenchsynthAudioProcessor) -> Self {
    
        todo!();
        /*
        : processor(p),

            //[Constructor_pre] You can add your own custom stuff here..
        //[/Constructor_pre]

        groupComponent.reset (new GroupComponent ("new group",
                                                  TRANS("Generate Mapping")));
        addAndMakeVisible (groupComponent.get());

        groupComponent->setBounds (4, 4, 292, 136);

        scaleStart.reset (new TextEditor ("new text editor"));
        addAndMakeVisible (scaleStart.get());
        scaleStart->setMultiLine (false);
        scaleStart->setReturnKeyStartsNewLine (false);
        scaleStart->setReadOnly (false);
        scaleStart->setScrollbarsShown (true);
        scaleStart->setCaretVisible (true);
        scaleStart->setPopupMenuEnabled (true);
        scaleStart->setText (TRANS("60"));

        scaleStart->setBounds (176, 24, 104, 24);

        tuneNote.reset (new TextEditor ("new text editor"));
        addAndMakeVisible (tuneNote.get());
        tuneNote->setMultiLine (false);
        tuneNote->setReturnKeyStartsNewLine (false);
        tuneNote->setReadOnly (false);
        tuneNote->setScrollbarsShown (true);
        tuneNote->setCaretVisible (true);
        tuneNote->setPopupMenuEnabled (true);
        tuneNote->setText (TRANS("69"));

        tuneNote->setBounds (176, 64, 104, 24);

        tuneFreq.reset (new TextEditor ("new text editor"));
        addAndMakeVisible (tuneFreq.get());
        tuneFreq->setMultiLine (false);
        tuneFreq->setReturnKeyStartsNewLine (false);
        tuneFreq->setReadOnly (false);
        tuneFreq->setScrollbarsShown (true);
        tuneFreq->setCaretVisible (true);
        tuneFreq->setPopupMenuEnabled (true);
        tuneFreq->setText (TRANS("440.0"));

        tuneFreq->setBounds (176, 104, 104, 24);

        applyButton.reset (new TextButton ("new button"));
        addAndMakeVisible (applyButton.get());
        applyButton->setButtonText (TRANS("Apply"));
        applyButton->addListener (this);

        applyButton->setBounds (192, 152, 102, 24);


        //[UserPreSize]
        //[/UserPreSize]

        setSize (300, 184);


        //[Constructor] You can add your own custom stuff here..
        //[/Constructor]
        */
    }
}

impl Drop for TWSKBMGenerator {

    fn drop(&mut self) {
        todo!();
        /*
            //[Destructor_pre]. You can add your own custom destruction code here..
        //[/Destructor_pre]

        groupComponent = nullptr;
        scaleStart = nullptr;
        tuneNote = nullptr;
        tuneFreq = nullptr;
        applyButton = nullptr;


        //[Destructor]. You can add your own custom destruction code here..
        //[/Destructor]
        */
    }
}

impl TWSKBMGenerator {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            //[UserPrePaint] Add your own custom painting code here..
        //[/UserPrePaint]

        g.fillAll (Colour (0xff323e44));

        {
            int x = 14, y = 20, width = 200, height = 30;
            String text (TRANS("Key for Scale 1/1:"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 14, y = 60, width = 200, height = 30;
            String text (TRANS("Reference Key:"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 14, y = 100, width = 200, height = 30;
            String text (TRANS("Reference Frequency (hz):"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        //[UserPaint] Add your own custom painting code here..
        //[/UserPaint]
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            //[UserPreResize] Add your own custom resize code here..
        //[/UserPreResize]

        //[UserResized] Add your own custom resize handling here..
        //[/UserResized]
        */
    }
    
    pub fn button_clicked(&mut self, button_that_was_clicked: *mut Button)  {
        
        todo!();
        /*
            //[UserbuttonClicked_Pre]
        //[/UserbuttonClicked_Pre]

        if (buttonThatWasClicked == applyButton.get())
        {
            //[UserButtonCode_applyButton] -- add your button handler code here..
            int ss = scaleStart->getText().getIntValue();
            int mn = tuneNote->getText().getIntValue();
            double fr = tuneFreq->getText().getDoubleValue();

            if( ss < 0 || ss > 127 || mn < 0 || mn > 127 || fr < 10 || fr > 10000 )
            {
                // Error condition
            }
            else
            {
                auto k = Tunings::startScaleOnAndTuneNoteTo( ss, mn, fr );
                processor.setKBM( k.rawText, true );
            }
            //[/UserButtonCode_applyButton]
        }

        //[UserbuttonClicked_Post]
        //[/UserbuttonClicked_Post]
        */
    }
}

