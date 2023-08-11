crate::ix!();

pub struct TWSAbout {
    version:        Box<Label>,
    builddate:      Box<Label>,
    builddate2:     Box<Label>,
    builddate3:     Box<Label>,
    builddate4:     Box<Label>,
    github_button:  Box<TextButton>,
    license_button: Box<TextButton>,
    team_button:    Box<TextButton>,
    drawable1:      Box<Drawable>,
}

impl Component      for TWSAbout {}
impl ButtonListener for TWSAbout {}

impl TWSAbout {

    //[UserMethods]     -- You can add your own custom methods in this section.
    //[/UserMethods]
    
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

impl TWSAbout {

    pub fn new() -> Self {
    
        todo!();
        /*


            //[Constructor_pre] You can add your own custom stuff here..
        //[/Constructor_pre]

        version.reset (new Label ("new label",
                                  TRANS("VERSION\n")));
        addAndMakeVisible (version.get());
        version->setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
        version->setJustificationType (Justification::centred);
        version->setEditable (false, false, false);
        version->setColour (TextEditor::textColourId, Colours::black);
        version->setColour (TextEditor::backgroundColourId, Colour (0x00000000));

        builddate.reset (new Label ("new label",
                                    TRANS("BUILD DATE\n")));
        addAndMakeVisible (builddate.get());
        builddate->setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
        builddate->setJustificationType (Justification::centred);
        builddate->setEditable (false, false, false);
        builddate->setColour (TextEditor::textColourId, Colours::black);
        builddate->setColour (TextEditor::backgroundColourId, Colour (0x00000000));

        builddate2.reset (new Label ("new label",
                                     TRANS("Released under GPL v3\n")));
        addAndMakeVisible (builddate2.get());
        builddate2->setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
        builddate2->setJustificationType (Justification::centred);
        builddate2->setEditable (false, false, false);
        builddate2->setColour (TextEditor::textColourId, Colours::black);
        builddate2->setColour (TextEditor::backgroundColourId, Colour (0x00000000));

        builddate3.reset (new Label ("new label",
                                     TRANS("Copyright 2019-2020\n")));
        addAndMakeVisible (builddate3.get());
        builddate3->setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
        builddate3->setJustificationType (Justification::centred);
        builddate3->setEditable (false, false, false);
        builddate3->setColour (TextEditor::textColourId, Colours::black);
        builddate3->setColour (TextEditor::backgroundColourId, Colour (0x00000000));

        builddate4.reset (new Label ("new label",
                                     TRANS("Various Authors\n")));
        addAndMakeVisible (builddate4.get());
        builddate4->setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
        builddate4->setJustificationType (Justification::centred);
        builddate4->setEditable (false, false, false);
        builddate4->setColour (TextEditor::textColourId, Colours::black);
        builddate4->setColour (TextEditor::backgroundColourId, Colour (0x00000000));

        githubButton.reset (new TextButton ("new button"));
        addAndMakeVisible (githubButton.get());
        githubButton->setButtonText (TRANS("Source on GitHub"));
        githubButton->addListener (this);

        githubButton->setBounds (25, 296, 158, 24);

        licenseButton.reset (new TextButton ("new button"));
        addAndMakeVisible (licenseButton.get());
        licenseButton->setButtonText (TRANS("Read the License"));
        licenseButton->addListener (this);

        licenseButton->setBounds (25, 328, 158, 24);

        teamButton.reset (new TextButton ("new button"));
        addAndMakeVisible (teamButton.get());
        teamButton->setButtonText (TRANS("About the Team"));
        teamButton->addListener (this);

        teamButton->setBounds (25, 360, 158, 24);

        drawable1 = Drawable::createFromImageData (BinaryData::TWSLogo_Inverted_NoBG_120_png, BinaryData::TWSLogo_Inverted_NoBG_120_pngSize);

        //[UserPreSize]
        auto vt = std::string( "version: " ) + Build::git_commit_hash + " branch " + Build::git_branch;
        version->setText( vt.c_str() , dontSendNotification );
        builddate->setText( "built: " __DATE__, dontSendNotification );
        //[/UserPreSize]

        setSize (208, 400);


        //[Constructor] You can add your own custom stuff here..
        //[/Constructor]
        */
    }
}

impl Drop for TWSAbout {

    fn drop(&mut self) {
        todo!();
        /*
            //[Destructor_pre]. You can add your own custom destruction code here..
        //[/Destructor_pre]

        version = nullptr;
        builddate = nullptr;
        builddate2 = nullptr;
        builddate3 = nullptr;
        builddate4 = nullptr;
        githubButton = nullptr;
        licenseButton = nullptr;
        teamButton = nullptr;
        drawable1 = nullptr;


        //[Destructor]. You can add your own custom destruction code here..
        //[/Destructor]
        */
    }
}

impl TWSAbout {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            //[UserPrePaint] Add your own custom painting code here..
        //[/UserPrePaint]

        g.fillAll (Colour (0xff323e44));

        {
            int x = 0, y = 112, width = proportionOfWidth (1.0000f), height = 27;
            String text (TRANS("Tuning Workbench"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (22.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 0, y = 135, width = proportionOfWidth (1.0000f), height = 30;
            String text (TRANS("A Surge Synth Team Product"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 40, y = 0, width = 120, height = 120;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (Colours::black);
            jassert (drawable1 != nullptr);
            if (drawable1 != nullptr)
                drawable1->drawWithin (g, Rectangle<int> (x, y, width, height).toFloat(),
                                       RectanglePlacement::stretchToFit, 1.000f);
        }

        //[UserPaint] Add your own custom painting code here..
        //[/UserPaint]
        */
    }
    
    pub fn resized(&mut self)  {
        //[UserPreResize] Add your own custom resize code here..
        //[/UserPreResize]

        /*
        (*version).set_bounds(0, 170, proportion_of_width(1.0000), 24);

        (*builddate).set_bounds(0, 194, proportion_of_width(1.0000), 24);

        (*builddate2).set_bounds(0, 226, proportion_of_width(1.0000), 24);

        (*builddate3).set_bounds(0, 244, proportion_of_width(1.0000), 24);

        (*builddate4).set_bounds(0, 263, proportion_of_width(1.0000), 24);
        */

        //[UserResized] Add your own custom resize handling here..
        //[/UserResized]
   }
    
    pub fn button_clicked(&mut self, button_that_was_clicked: *mut Button)  {
        
        todo!();
        /*
            //[UserbuttonClicked_Pre]
        //[/UserbuttonClicked_Pre]

        if (buttonThatWasClicked == githubButton.get())
        {
            //[UserButtonCode_githubButton] -- add your button handler code here..
            URL( "https://github.com/surge-synthesizer/tuning-workbench-synth/" ).launchInDefaultBrowser();
            //[/UserButtonCode_githubButton]
        }
        else if (buttonThatWasClicked == licenseButton.get())
        {
            //[UserButtonCode_licenseButton] -- add your button handler code here..
            URL( "https://www.gnu.org/licenses/gpl-3.0.en.html" ).launchInDefaultBrowser();
            //[/UserButtonCode_licenseButton]
        }
        else if (buttonThatWasClicked == teamButton.get())
        {
            //[UserButtonCode_teamButton] -- add your button handler code here..
            URL( "https://surge-synth-team.org/" ).launchInDefaultBrowser();
            //[/UserButtonCode_teamButton]
        }

        //[UserbuttonClicked_Post]
        //[/UserbuttonClicked_Post]
        */
    }
}

