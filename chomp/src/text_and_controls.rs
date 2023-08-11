crate::ix!();

/**
  | An auto-generated component, created
  | by the Projucer.
  | 
  | Describe your class and how it works
  | here!
  |
  */
pub struct TWSTextAndControls {
    is_scl:                    bool,
    supress_first_text_change: bool,
    processor:                 &mut TuningworkbenchsynthAudioProcessor,
    advanced_window:           *mut ScaleEditorWindow, // default = nullptr
    text_editor:               Box<TextEditor>,
    apply_button:              Box<TextButton>,
    open_button:               Box<TextButton>,
    export_button:             Box<TextButton>,
    reset_button:              Box<TextButton>,
    adv_button:                Box<TextButton>,
}

impl Component               for TWSTextAndControls {}
impl TextEditorListener      for TWSTextAndControls {}
impl ScaleTextEditedListener for TWSTextAndControls {}
impl NotesOnChangedListener  for TWSTextAndControls {}
impl ButtonListener          for TWSTextAndControls {}

impl TuningUpdatedListener for TWSTextAndControls {

    pub fn tuning_updated(&mut self, new_tuning: &Tuning)  {
        
        todo!();
        /*
        
        */
    }
}

impl TWSTextAndControls {
    
    pub fn new(
        is_scl: bool,
        p:      &mut TuningworkbenchsynthAudioProcessor) -> Self {
    
        todo!();
        /*


        
        */
    }
    
    pub fn text_editor_text_changed(&mut self, t: &mut TextEditor)  {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn scale_text_edited(&mut self, new_scale: String)  {
        
        todo!();
        /*
            if( isSCL )
                processor.setSCL( newScale );
        */
    }
    
    pub fn note_on(&mut self, note_num: i32)  {
        
        todo!();
        /*
            if( advancedWindow )
            {
                int scalenote = processor.tuning.scalePositionForMidiNote(noteNum);
                advancedWindow->editor->scaleNoteOn( scalenote );
            }
        */
    }
    
    pub fn note_off(&mut self, note_num: i32)  {
        
        todo!();
        /*
            if( advancedWindow )
            {
                int scalenote = processor.tuning.scalePositionForMidiNote(noteNum);
                advancedWindow->editor->scaleNoteOff( scalenote );
            }
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

impl TWSTextAndControls {
    
    pub fn new(
        is_scl: bool,
        p:      &mut TuningworkbenchsynthAudioProcessor) -> Self {
    
        todo!();
        /*
        : processor(p),

            //[Constructor_pre] You can add your own custom stuff here..
        this->isSCL = isSCL;
        //[/Constructor_pre]

        textEditor.reset (new TextEditor ("new text editor"));
        addAndMakeVisible (textEditor.get());
        textEditor->setMultiLine (true);
        textEditor->setReturnKeyStartsNewLine (true);
        textEditor->setReadOnly (false);
        textEditor->setScrollbarsShown (true);
        textEditor->setCaretVisible (true);
        textEditor->setPopupMenuEnabled (true);
        textEditor->setText (String());

        applyButton.reset (new TextButton ("new button"));
        addAndMakeVisible (applyButton.get());
        applyButton->setButtonText (TRANS("Apply"));
        applyButton->addListener (this);

        applyButton->setBounds (346, 298, 78, 24);

        openButton.reset (new TextButton ("open file"));
        addAndMakeVisible (openButton.get());
        openButton->setButtonText (TRANS("Open"));
        openButton->addListener (this);

        openButton->setBounds (94, 298, 78, 24);

        exportButton.reset (new TextButton ("export file"));
        addAndMakeVisible (exportButton.get());
        exportButton->setButtonText (TRANS("Export"));
        exportButton->addListener (this);

        exportButton->setBounds (10, 298, 78, 24);

        resetButton.reset (new TextButton ("reset"));
        addAndMakeVisible (resetButton.get());
        resetButton->setButtonText (TRANS("Reset"));
        resetButton->addListener (this);

        resetButton->setBounds (262, 298, 78, 24);

        advButton.reset (new TextButton ("open file"));
        addAndMakeVisible (advButton.get());
        advButton->setButtonText (TRANS("Advanced"));
        advButton->addListener (this);

        advButton->setBounds (178, 298, 78, 24);


        //[UserPreSize]
        textEditor->setMultiLine (true, false); // turn off word wrap
        supressFirstTextChange = true;
        //[/UserPreSize]

        setSize (432, 328);


        //[Constructor] You can add your own custom stuff here..
        auto tf = Typeface::createSystemTypefaceFor(BinaryData::FiraCodeRegular_ttf, BinaryData::FiraCodeRegular_ttfSize);
        Font fira(tf);
        fira.setHeight(15);
        textEditor->setFont(fira);

        if( isSCL )
        {
            applyButton->setButtonText( TRANS( "Apply" ) );
            openButton->setButtonText( TRANS( "Load" ) );
        }
        else
        {
            applyButton->setButtonText( TRANS( "Apply" ) );
            openButton->setButtonText( TRANS( "Load" ) );
            advButton->setButtonText( TRANS( "Generate" ) );
        }
        applyButton->setEnabled( false );
        textEditor->addListener( this );
        //[/Constructor]
        */
    }
}

impl Drop for TWSTextAndControls {
    fn drop(&mut self) {
        todo!();
        /*
            //[Destructor_pre]. You can add your own custom destruction code here..
        //[/Destructor_pre]

        textEditor = nullptr;
        applyButton = nullptr;
        openButton = nullptr;
        exportButton = nullptr;
        resetButton = nullptr;
        advButton = nullptr;


        //[Destructor]. You can add your own custom destruction code here..
        //[/Destructor]
        */
    }
}

impl TWSTextAndControls {
    
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

        textEditor->setBounds (0, 0, proportionOfWidth (1.0000f), 288);
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
            if( isSCL )
                processor.setSCL( textEditor->getText() );
            else
                processor.setKBM( textEditor->getText() );
            //[/UserButtonCode_applyButton]
        }
        else if (buttonThatWasClicked == openButton.get())
        {
            //[UserButtonCode_openButton] -- add your button handler code here..
            FileChooser fc( isSCL ? "Open SCL File" : "Open KBM File", File(), isSCL ? "*.scl" : "*.kbm" );
            if( fc.browseForFileToOpen() )
            {
                auto s = fc.getResult().loadFileAsString();
                processor.setSCL(s);
            }
            //[/UserButtonCode_openButton]
        }
        else if (buttonThatWasClicked == exportButton.get())
        {
            //[UserButtonCode_exportButton] -- add your button handler code here..
            FileChooser fc( isSCL ? "Export SCL File" : "Export KBM File", File(), isSCL ? "*.scl" : "*.kbm" );
            if( fc.browseForFileToSave(true) )
            {
                auto f = fc.getResult();
                if( ! f.replaceWithText( textEditor->getText() ) )
                {
                    AlertWindow::showMessageBoxAsync( AlertWindow::AlertIconType::WarningIcon,
                                                      "Error exporting file",
                                                      "An unknown error occured streaming data to file",
                                                      "OK" );

                }
            }
            //[/UserButtonCode_exportButton]
        }
        else if (buttonThatWasClicked == resetButton.get())
        {
            //[UserButtonCode_resetButton] -- add your button handler code here..
            if( isSCL )
                processor.resetSCLToStandard();
            else
                processor.resetKBMToStandard();
            //[/UserButtonCode_resetButton]
        }
        else if (buttonThatWasClicked == advButton.get())
        {
            //[UserButtonCode_advButton] -- add your button handler code here..
            if( isSCL )
            {
                if( !advancedWindow )
                {
                    auto ed = new surgesynthteam::ScaleEditor(processor.tuning.scale);
                    ed->addScaleTextEditedListener( this );
                    advancedWindow = new surgesynthteam::ScaleEditorWindow( ed );
                    advancedWindow->onCloseButton = [this](){ this->advancedWindow = nullptr; };
                    advancedWindow->setVisible( true );
                }
            }
            else
            {
                auto ed = new TWSKBMGenerator(processor);
                DialogWindow::LaunchOptions options;
                options.content.setOwned(ed);
                options.dialogTitle = "KBM Generator";
                options.escapeKeyTriggersCloseButton = true;
                options.useNativeTitleBar = false;
                options.resizable = false;

                options.launchAsync();
            }
            //[/UserButtonCode_advButton]
        }

        //[UserbuttonClicked_Post]
        //[/UserbuttonClicked_Post]
        */
    }

    pub fn tuning_updated(&mut self, new_tuning: &Tuning)  {
        
        todo!();
        /*
            if( isSCL )
        {
            textEditor->setText( newTuning.scale.rawText, false );
            if( advancedWindow )
            {
                advancedWindow->editor->resetScale( newTuning.scale );
            }
        }
        else
        {
            textEditor->setText( newTuning.keyboardMapping.rawText, false );
        }
        applyButton->setEnabled( false );
        */
    }
    
    pub fn text_editor_text_changed(&mut self, t: &mut TextEditor)  {
        
        todo!();
        /*
            if( supressFirstTextChange )
        {
            supressFirstTextChange = false;
            return;
        }

        applyButton->setEnabled( true  );
        */
    }
}
