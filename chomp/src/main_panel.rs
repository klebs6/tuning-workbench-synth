crate::ix!();

pub struct TWSPowerToggle {

}

impl ToggleButtonInterface for TWSPowerToggle {}

impl Default for TWSPowerToggle {
    
    fn default() -> Self {
        todo!();
        /*
        : toggle_button(),

            }{
        */
    }
}

impl TWSPowerToggle {
    
    pub fn paint_button(&mut self, 
        g:  &mut Graphics,
        hl: bool,
        dn: bool)  {
        
        todo!();
        /*
        
        */
    }
}

pub struct TWSLambdaParamListener {
    lfunc: fn(_0: i32, _1: f32) -> (),
}

impl AudioProcessorParameterListener for TWSLambdaParamListener {

}

impl TWSLambdaParamListener {
    
    pub fn parameter_gesture_changed(&mut self, 
        _0: i32,
        _1: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn parameter_value_changed(&mut self, 
        idx: i32,
        nv:  f32)  {
        
        todo!();
        /*
            lfunc(idx, nv);
        */
    }
}

/**
  | An auto-generated component, created
  | by the Projucer.
  | 
  | Describe your class and how it works
  | here!
  |
  */
pub struct TWSMainPanel {
    processor:             &mut TuningworkbenchsynthAudioProcessor,
    scl_text_and_controls: TWSTextAndControlsPtr,
    kbm_text_and_controls: TWSTextAndControlsPtr,
    slider_attachments:    Vec<Box<SliderAttachment>>,
    button_attachments:    Vec<Box<ButtonAttachment>>,
    lambda_atttachments:   Vec<Pair<String,*mut TWSLambdaParamListener>>,
    preset_menu:           Box<PopupMenu>,
    version3:              Box<Label>,
    group_component7:      Box<GroupComponent>,
    group_component8:      Box<GroupComponent>,
    tuning_grid:           Box<TWSTuningGrid>,
    group_component3:      Box<GroupComponent>,
    group_component2:      Box<GroupComponent>,
    group_component:       Box<GroupComponent>,
    square_mix:            Box<Slider>,
    tabbed_component:      Box<TabbedComponent>,
    sine_mix:              Box<Slider>,
    saw_mix:               Box<Slider>,
    tri_mix:               Box<Slider>,
    AEG_A:                 Box<Slider>,
    FEG_A:                 Box<Slider>,
    AEG_D:                 Box<Slider>,
    FEG_D:                 Box<Slider>,
    AEG_S:                 Box<Slider>,
    FEG_S:                 Box<Slider>,
    AEG_R:                 Box<Slider>,
    FEG_R:                 Box<Slider>,
    group_component4:      Box<GroupComponent>,
    filt_cutoff:           Box<Slider>,
    filt_q:                Box<Slider>,
    lpf_toggle:            Box<ToggleButton>,
    hpf_toggle:            Box<ToggleButton>,
    bpf_toggle:            Box<ToggleButton>,
    group_component5:      Box<GroupComponent>,
    master_sat:            Box<Slider>,
    master_out:            Box<Slider>,
    feg_depth:             Box<Slider>,
    uni_spread:            Box<Slider>,
    uni_count:             Box<Slider>,
    pb_up:                 Box<Slider>,
    pb_down:               Box<Slider>,
    version:               Box<Label>,
    about_button:          Box<TextButton>,
    lfo_rate:              Box<Slider>,
    lfo_delay:             Box<Slider>,
    lfo_attack:            Box<Slider>,
    lfotritog:             Box<ToggleButton>,
    lfosqrtog:             Box<ToggleButton>,
    lforndtog:             Box<ToggleButton>,
    group_component9:      Box<GroupComponent>,
    sublevel:              Box<Slider>,
    sub_oct:               Box<Slider>,
    group_component10:     Box<GroupComponent>,
    group_component11:     Box<GroupComponent>,
    delay_time:            Box<Slider>,
    pluck_atn:             Box<Slider>,
    pluck_flt:             Box<Slider>,
    pluck_init:            Box<Slider>,
    pluck_lev:             Box<Slider>,
    sub_power:             Box<TWSPowerToggle>,
    pluck_power:           Box<TWSPowerToggle>,
    delay_power:           Box<TWSPowerToggle>,
    vco_power:             Box<TWSPowerToggle>,
    wheel_lab:             Box<Label>,
    mod_wheel_power:       Box<TWSPowerToggle>,
    group_component6:      Box<GroupComponent>,
    filter_power:          Box<TWSPowerToggle>,
    delay_fb:              Box<Slider>,
    delay_wet:             Box<Slider>,
    delay_dry:             Box<Slider>,
    lfo_pitch:             Box<Slider>,
    lfo_sublev:            Box<Slider>,
    lfo_vcolev:            Box<Slider>,
    lfo_plucklev:          Box<Slider>,
    lfo_filter:            Box<Slider>,
    version2:              Box<Label>,
    help_button:           Box<TextButton>,
    preset_button:         Box<TextButton>,
}

impl Component for TWSMainPanel {

}

impl FileDragAndDropTarget for TWSMainPanel {

}

impl ButtonListener for TWSMainPanel {

}

impl SliderListener for TWSMainPanel {

}

pub type SliderAttachment = AudioProcessorValueTreeState_SliderAttachment;
pub type ButtonAttachment = AudioProcessorValueTreeState_ButtonAttachment;

impl TWSMainPanel {
    
    pub fn new(p: &mut TuningworkbenchsynthAudioProcessor) -> Self {
    
        todo!();
        /*


        
        */
    }
    
    pub fn connect_value_tree_state(&mut self, p: &mut AudioProcessorValueTreeState)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_interested_in_file_drag(&mut self, files: &StringArray) -> bool {
        
        todo!();
        /*
            return true; // FIXME
        */
    }
    
    pub fn do_preset_menu(&mut self)  {
        
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
    
    pub fn slider_value_changed(&mut self, slider_that_was_moved: *mut Slider)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn files_dropped(&mut self, 
        filenames: &StringArray,
        mousex:    i32,
        mousey:    i32)  {
        
        todo!();
        /*
        
        */
    }
}

//==============================================================================
impl TWSMainPanel {
    
    pub fn new(p: &mut TuningworkbenchsynthAudioProcessor) -> Self {
    
        todo!();
        /*
        : processor(p),

            //[Constructor_pre] You can add your own custom stuff here..
        //[/Constructor_pre]

        version3.reset (new Label ("Version Label",
                                   TRANS("https://surge-synth-team.org/")));
        addAndMakeVisible (version3.get());
        version3->setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
        version3->setJustificationType (Justification::centred);
        version3->setEditable (false, false, false);
        version3->setColour (Label::textColourId, Colours::white);
        version3->setColour (TextEditor::textColourId, Colours::black);
        version3->setColour (TextEditor::backgroundColourId, Colour (0x00000000));

        version3->setBounds (0, 721, 784, 16);

        groupComponent7.reset (new GroupComponent ("new group",
                                                   TRANS("Tuning")));
        addAndMakeVisible (groupComponent7.get());

        groupComponent7->setBounds (8, 320, 768, 400);

        groupComponent8.reset (new GroupComponent ("new group",
                                                   TRANS("LFO")));
        addAndMakeVisible (groupComponent8.get());

        groupComponent8->setBounds (224, 224, 400, 96);

        tuningGrid.reset (new TWSTuningGrid());
        addAndMakeVisible (tuningGrid.get());
        tuningGrid->setName ("tuningGrid");

        tuningGrid->setBounds (24, 344, 296, 360);

        groupComponent3.reset (new GroupComponent ("new group",
                                                   TRANS("Filter Envelope")));
        addAndMakeVisible (groupComponent3.get());

        groupComponent3->setBounds (224, 32, 336, 96);

        groupComponent2.reset (new GroupComponent ("new group",
                                                   TRANS("Amplitude Envelope")));
        addAndMakeVisible (groupComponent2.get());

        groupComponent2->setBounds (224, 128, 272, 96);

        groupComponent.reset (new GroupComponent ("new group",
                                                  TRANS("VCO")));
        addAndMakeVisible (groupComponent.get());

        groupComponent->setBounds (8, 32, 208, 128);

        squareMix.reset (new Slider ("squareMix"));
        addAndMakeVisible (squareMix.get());
        squareMix->setRange (0, 1, 0);
        squareMix->setSliderStyle (Slider::LinearHorizontal);
        squareMix->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);

        squareMix->setBounds (48, 72, 88, 30);

        tabbedComponent.reset (new TabbedComponent (TabbedButtonBar::TabsAtTop));
        addAndMakeVisible (tabbedComponent.get());
        tabbedComponent->setTabBarDepth (30);
        tabbedComponent->addTab (TRANS("SCL File"), Colours::lightgrey, new TWSTextAndControls (true, processor), true);
        tabbedComponent->addTab (TRANS("KBM File"), Colours::lightgrey, new TWSTextAndControls (false, processor), true);
        tabbedComponent->setCurrentTabIndex (0);

        tabbedComponent->setBounds (328, 344, 432, 360);

        sineMix.reset (new Slider ("sineMix"));
        addAndMakeVisible (sineMix.get());
        sineMix->setRange (0, 1, 0);
        sineMix->setSliderStyle (Slider::LinearHorizontal);
        sineMix->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);

        sineMix->setBounds (48, 48, 88, 30);

        sawMix.reset (new Slider ("sawMix"));
        addAndMakeVisible (sawMix.get());
        sawMix->setRange (0, 1, 0);
        sawMix->setSliderStyle (Slider::LinearHorizontal);
        sawMix->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);

        sawMix->setBounds (48, 96, 88, 30);

        triMix.reset (new Slider ("triMix"));
        addAndMakeVisible (triMix.get());
        triMix->setRange (0, 1, 0);
        triMix->setSliderStyle (Slider::LinearHorizontal);
        triMix->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);

        triMix->setBounds (48, 120, 88, 30);

        AEG_A.reset (new Slider ("aeg_A"));
        addAndMakeVisible (AEG_A.get());
        AEG_A->setRange (0, 10, 0);
        AEG_A->setSliderStyle (Slider::RotaryVerticalDrag);
        AEG_A->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        AEG_A->setBounds (232, 142, 63, 72);

        FEG_A.reset (new Slider ("feg_A"));
        addAndMakeVisible (FEG_A.get());
        FEG_A->setRange (0, 10, 0);
        FEG_A->setSliderStyle (Slider::RotaryVerticalDrag);
        FEG_A->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        FEG_A->setBounds (232, 46, 63, 72);

        AEG_D.reset (new Slider ("aeg_D"));
        addAndMakeVisible (AEG_D.get());
        AEG_D->setRange (0, 10, 0);
        AEG_D->setSliderStyle (Slider::RotaryVerticalDrag);
        AEG_D->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        AEG_D->setBounds (296, 142, 63, 72);

        FEG_D.reset (new Slider ("feg_D"));
        addAndMakeVisible (FEG_D.get());
        FEG_D->setRange (0, 10, 0);
        FEG_D->setSliderStyle (Slider::RotaryVerticalDrag);
        FEG_D->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        FEG_D->setBounds (296, 46, 63, 72);

        AEG_S.reset (new Slider ("aeg_S"));
        addAndMakeVisible (AEG_S.get());
        AEG_S->setRange (0, 10, 0);
        AEG_S->setSliderStyle (Slider::RotaryVerticalDrag);
        AEG_S->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        AEG_S->setBounds (360, 142, 63, 72);

        FEG_S.reset (new Slider ("feg_S"));
        addAndMakeVisible (FEG_S.get());
        FEG_S->setRange (0, 10, 0);
        FEG_S->setSliderStyle (Slider::RotaryVerticalDrag);
        FEG_S->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        FEG_S->setBounds (360, 46, 63, 72);

        AEG_R.reset (new Slider ("aeg_R"));
        addAndMakeVisible (AEG_R.get());
        AEG_R->setRange (0, 10, 0);
        AEG_R->setSliderStyle (Slider::RotaryVerticalDrag);
        AEG_R->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        AEG_R->setBounds (424, 142, 63, 72);

        FEG_R.reset (new Slider ("feg_R"));
        addAndMakeVisible (FEG_R.get());
        FEG_R->setRange (0, 10, 0);
        FEG_R->setSliderStyle (Slider::RotaryVerticalDrag);
        FEG_R->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        FEG_R->setBounds (424, 46, 63, 72);

        groupComponent4.reset (new GroupComponent ("new group",
                                                   TRANS("Filter")));
        addAndMakeVisible (groupComponent4.get());

        groupComponent4->setBounds (568, 32, 208, 96);

        Filt_Cutoff.reset (new Slider ("filt_cutoff"));
        addAndMakeVisible (Filt_Cutoff.get());
        Filt_Cutoff->setRange (0, 10, 0);
        Filt_Cutoff->setSliderStyle (Slider::RotaryVerticalDrag);
        Filt_Cutoff->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        Filt_Cutoff->setBounds (641, 45, 63, 72);

        Filt_Q.reset (new Slider ("fllt_Q"));
        addAndMakeVisible (Filt_Q.get());
        Filt_Q->setRange (0, 10, 0);
        Filt_Q->setSliderStyle (Slider::RotaryVerticalDrag);
        Filt_Q->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        Filt_Q->setBounds (705, 45, 63, 72);

        LPFToggle.reset (new ToggleButton ("LPF"));
        addAndMakeVisible (LPFToggle.get());
        LPFToggle->setRadioGroupId (1);
        LPFToggle->addListener (this);
        LPFToggle->setToggleState (true, dontSendNotification);

        LPFToggle->setBounds (576, 50, 56, 24);

        HPFToggle.reset (new ToggleButton ("HPF"));
        addAndMakeVisible (HPFToggle.get());
        HPFToggle->setRadioGroupId (1);
        HPFToggle->addListener (this);

        HPFToggle->setBounds (576, 71, 56, 24);

        BPFToggle.reset (new ToggleButton ("BPF"));
        addAndMakeVisible (BPFToggle.get());
        BPFToggle->setRadioGroupId (1);
        BPFToggle->addListener (this);

        BPFToggle->setBounds (576, 93, 56, 24);

        groupComponent5.reset (new GroupComponent ("new group",
                                                   TRANS("Master")));
        addAndMakeVisible (groupComponent5.get());

        groupComponent5->setBounds (632, 224, 144, 96);

        master_sat.reset (new Slider ("master_sat"));
        addAndMakeVisible (master_sat.get());
        master_sat->setRange (0, 10, 0);
        master_sat->setSliderStyle (Slider::RotaryVerticalDrag);
        master_sat->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        master_sat->setBounds (641, 236, 63, 72);

        master_out.reset (new Slider ("master_out"));
        addAndMakeVisible (master_out.get());
        master_out->setRange (0, 10, 0);
        master_out->setSliderStyle (Slider::RotaryVerticalDrag);
        master_out->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        master_out->setBounds (705, 236, 63, 72);

        FEG_depth.reset (new Slider ("feg_depth"));
        addAndMakeVisible (FEG_depth.get());
        FEG_depth->setRange (0, 10, 0);
        FEG_depth->setSliderStyle (Slider::RotaryVerticalDrag);
        FEG_depth->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        FEG_depth->setBounds (487, 46, 63, 72);

        uni_spread.reset (new Slider ("Uni_Spread"));
        addAndMakeVisible (uni_spread.get());
        uni_spread->setRange (0, 10, 0);
        uni_spread->setSliderStyle (Slider::LinearHorizontal);
        uni_spread->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        uni_spread->setBounds (152, 112, 56, 40);

        uni_count.reset (new Slider ("Uni Count"));
        addAndMakeVisible (uni_count.get());
        uni_count->setRange (0, 10, 0);
        uni_count->setSliderStyle (Slider::IncDecButtons);
        uni_count->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        uni_count->setBounds (152, 64, 56, 32);

        pb_up.reset (new Slider ("PB UP"));
        addAndMakeVisible (pb_up.get());
        pb_up->setRange (0, 10, 0);
        pb_up->setSliderStyle (Slider::IncDecButtons);
        pb_up->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        pb_up->setBounds (550, 151, 56, 30);

        pb_down.reset (new Slider ("PB Down"));
        addAndMakeVisible (pb_down.get());
        pb_down->setRange (0, 10, 0);
        pb_down->setSliderStyle (Slider::IncDecButtons);
        pb_down->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        pb_down->setBounds (550, 183, 58, 30);

        version.reset (new Label ("Version Label",
                                  TRANS("set-this")));
        addAndMakeVisible (version.get());
        version->setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
        version->setJustificationType (Justification::centredRight);
        version->setEditable (false, false, false);
        version->setColour (Label::textColourId, Colour (0xffa6a6a6));
        version->setColour (TextEditor::textColourId, Colours::black);
        version->setColour (TextEditor::backgroundColourId, Colour (0x00000000));

        version->setBounds (592, 721, 190, 16);

        aboutButton.reset (new TextButton ("aboutButton"));
        addAndMakeVisible (aboutButton.get());
        aboutButton->setButtonText (TRANS("About"));
        aboutButton->addListener (this);

        aboutButton->setBounds (716, 3, 55, 24);

        lfo_rate.reset (new Slider ("lfo_rate"));
        addAndMakeVisible (lfo_rate.get());
        lfo_rate->setRange (0, 10, 0);
        lfo_rate->setSliderStyle (Slider::RotaryVerticalDrag);
        lfo_rate->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        lfo_rate->setBounds (304, 238, 63, 72);

        lfo_delay.reset (new Slider ("lfo_delay"));
        addAndMakeVisible (lfo_delay.get());
        lfo_delay->setRange (0, 10, 0);
        lfo_delay->setSliderStyle (Slider::RotaryVerticalDrag);
        lfo_delay->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        lfo_delay->setBounds (368, 238, 63, 72);

        lfo_attack.reset (new Slider ("lfo_attack"));
        addAndMakeVisible (lfo_attack.get());
        lfo_attack->setRange (0, 10, 0);
        lfo_attack->setSliderStyle (Slider::RotaryVerticalDrag);
        lfo_attack->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        lfo_attack->setBounds (432, 238, 63, 72);

        lfotritog.reset (new ToggleButton ("lfotri"));
        addAndMakeVisible (lfotritog.get());
        lfotritog->setButtonText (TRANS("TRI"));
        lfotritog->setRadioGroupId (2);
        lfotritog->addListener (this);
        lfotritog->setToggleState (true, dontSendNotification);

        lfotritog->setBounds (237, 242, 56, 24);

        lfosqrtog.reset (new ToggleButton ("lfosq"));
        addAndMakeVisible (lfosqrtog.get());
        lfosqrtog->setButtonText (TRANS("SQR"));
        lfosqrtog->setRadioGroupId (2);
        lfosqrtog->addListener (this);

        lfosqrtog->setBounds (237, 263, 56, 24);

        lforndtog.reset (new ToggleButton ("lfornd"));
        addAndMakeVisible (lforndtog.get());
        lforndtog->setButtonText (TRANS("RND"));
        lforndtog->setRadioGroupId (2);
        lforndtog->addListener (this);

        lforndtog->setBounds (237, 285, 56, 24);

        groupComponent9.reset (new GroupComponent ("new group",
                                                   TRANS("Sub")));
        addAndMakeVisible (groupComponent9.get());

        groupComponent9->setBounds (8, 160, 208, 64);

        sublevel.reset (new Slider ("sineMix"));
        addAndMakeVisible (sublevel.get());
        sublevel->setRange (0, 1, 0);
        sublevel->setSliderStyle (Slider::LinearHorizontal);
        sublevel->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);

        sublevel->setBounds (18, 179, 94, 30);

        sub_oct.reset (new Slider ("Octave"));
        addAndMakeVisible (sub_oct.get());
        sub_oct->setRange (0, 10, 0);
        sub_oct->setSliderStyle (Slider::IncDecButtons);
        sub_oct->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        sub_oct->setBounds (148, 177, 56, 32);

        groupComponent10.reset (new GroupComponent ("new group",
                                                    TRANS("Pluck")));
        addAndMakeVisible (groupComponent10.get());

        groupComponent10->setBounds (8, 224, 208, 96);

        groupComponent11.reset (new GroupComponent ("new group",
                                                    TRANS("Delay")));
        addAndMakeVisible (groupComponent11.get());

        groupComponent11->setBounds (632, 128, 144, 96);

        delay_time.reset (new Slider ("delay_time"));
        addAndMakeVisible (delay_time.get());
        delay_time->setRange (0, 10, 0);
        delay_time->setSliderStyle (Slider::RotaryVerticalDrag);
        delay_time->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        delay_time->setBounds (640, 144, 63, 72);

        pluck_atn.reset (new Slider ("squareMix"));
        addAndMakeVisible (pluck_atn.get());
        pluck_atn->setRange (0, 1, 0);
        pluck_atn->setSliderStyle (Slider::LinearHorizontal);
        pluck_atn->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);

        pluck_atn->setBounds (48, 264, 88, 30);

        pluck_flt.reset (new Slider ("sineMix"));
        addAndMakeVisible (pluck_flt.get());
        pluck_flt->setRange (0, 1, 0);
        pluck_flt->setSliderStyle (Slider::LinearHorizontal);
        pluck_flt->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);

        pluck_flt->setBounds (48, 240, 88, 30);

        pluck_init.reset (new Slider ("sawMix"));
        addAndMakeVisible (pluck_init.get());
        pluck_init->setRange (0, 1, 0);
        pluck_init->setSliderStyle (Slider::LinearHorizontal);
        pluck_init->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);

        pluck_init->setBounds (48, 288, 88, 30);

        pluck_lev.reset (new Slider ("lfo_rate"));
        addAndMakeVisible (pluck_lev.get());
        pluck_lev->setRange (0, 10, 0);
        pluck_lev->setSliderStyle (Slider::RotaryVerticalDrag);
        pluck_lev->setTextBoxStyle (Slider::TextBoxBelow, false, 60, 15);

        pluck_lev->setBounds (144, 240, 63, 72);

        SubPower.reset (new TWSPowerToggle());
        addAndMakeVisible (SubPower.get());
        SubPower->setName ("new component");

        SubPower->setBounds (184, 160, 16, 16);

        PluckPower.reset (new TWSPowerToggle());
        addAndMakeVisible (PluckPower.get());
        PluckPower->setName ("new component");

        PluckPower->setBounds (184, 224, 16, 16);

        DelayPower.reset (new TWSPowerToggle());
        addAndMakeVisible (DelayPower.get());
        DelayPower->setName ("new component");

        DelayPower->setBounds (744, 128, 16, 16);

        VCOPower.reset (new TWSPowerToggle());
        addAndMakeVisible (VCOPower.get());
        VCOPower->setName ("new component");

        VCOPower->setBounds (184, 32, 16, 16);

        wheelLab.reset (new Label ("new label",
                                   TRANS("wheel")));
        addAndMakeVisible (wheelLab.get());
        wheelLab->setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
        wheelLab->setJustificationType (Justification::centred);
        wheelLab->setEditable (false, false, false);
        wheelLab->setColour (Label::backgroundColourId, Colour (0xffab1d1d));
        wheelLab->setColour (TextEditor::textColourId, Colours::black);
        wheelLab->setColour (TextEditor::backgroundColourId, Colour (0x00000000));

        wheelLab->setBounds (545, 219, 55, 24);

        ModWheelPower.reset (new TWSPowerToggle());
        addAndMakeVisible (ModWheelPower.get());
        ModWheelPower->setName ("new component");

        ModWheelPower->setBounds (593, 224, 16, 16);

        groupComponent6.reset (new GroupComponent ("new group",
                                                   TRANS("Bend")));
        addAndMakeVisible (groupComponent6.get());

        groupComponent6->setBounds (504, 128, 120, 96);

        FilterPower.reset (new TWSPowerToggle());
        addAndMakeVisible (FilterPower.get());
        FilterPower->setName ("new component");

        FilterPower->setBounds (744, 32, 16, 16);

        delay_fb.reset (new Slider ("new slider"));
        addAndMakeVisible (delay_fb.get());
        delay_fb->setRange (0, 10, 0);
        delay_fb->setSliderStyle (Slider::LinearVertical);
        delay_fb->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);
        delay_fb->addListener (this);

        delay_fb->setBounds (702, 151, 24, 68);

        delay_wet.reset (new Slider ("new slider"));
        addAndMakeVisible (delay_wet.get());
        delay_wet->setRange (0, 10, 0);
        delay_wet->setSliderStyle (Slider::LinearVertical);
        delay_wet->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);
        delay_wet->addListener (this);

        delay_wet->setBounds (748, 151, 24, 68);

        delay_dry.reset (new Slider ("new slider"));
        addAndMakeVisible (delay_dry.get());
        delay_dry->setRange (0, 10, 0);
        delay_dry->setSliderStyle (Slider::LinearVertical);
        delay_dry->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);
        delay_dry->addListener (this);

        delay_dry->setBounds (725, 151, 24, 68);

        lfo_pitch.reset (new Slider ("new slider"));
        addAndMakeVisible (lfo_pitch.get());
        lfo_pitch->setRange (0, 10, 0);
        lfo_pitch->setSliderStyle (Slider::LinearVertical);
        lfo_pitch->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);
        lfo_pitch->addListener (this);

        lfo_pitch->setBounds (496, 247, 24, 68);

        lfo_sublev.reset (new Slider ("new slider"));
        addAndMakeVisible (lfo_sublev.get());
        lfo_sublev->setRange (0, 10, 0);
        lfo_sublev->setSliderStyle (Slider::LinearVertical);
        lfo_sublev->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);
        lfo_sublev->addListener (this);

        lfo_sublev->setBounds (544, 247, 24, 68);

        lfo_vcolev.reset (new Slider ("new slider"));
        addAndMakeVisible (lfo_vcolev.get());
        lfo_vcolev->setRange (0, 10, 0);
        lfo_vcolev->setSliderStyle (Slider::LinearVertical);
        lfo_vcolev->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);
        lfo_vcolev->addListener (this);

        lfo_vcolev->setBounds (520, 247, 24, 68);

        lfo_plucklev.reset (new Slider ("new slider"));
        addAndMakeVisible (lfo_plucklev.get());
        lfo_plucklev->setRange (0, 10, 0);
        lfo_plucklev->setSliderStyle (Slider::LinearVertical);
        lfo_plucklev->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);
        lfo_plucklev->addListener (this);

        lfo_plucklev->setBounds (568, 247, 24, 68);

        lfo_filter.reset (new Slider ("new slider"));
        addAndMakeVisible (lfo_filter.get());
        lfo_filter->setRange (0, 10, 0);
        lfo_filter->setSliderStyle (Slider::LinearVertical);
        lfo_filter->setTextBoxStyle (Slider::NoTextBox, false, 80, 20);
        lfo_filter->addListener (this);

        lfo_filter->setBounds (592, 247, 24, 68);

        version2.reset (new Label ("Version Label",
                                   TRANS("Released under GNU General Public License v3\n")));
        addAndMakeVisible (version2.get());
        version2->setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
        version2->setJustificationType (Justification::centredLeft);
        version2->setEditable (false, false, false);
        version2->setColour (Label::textColourId, Colour (0xffa6a6a6));
        version2->setColour (TextEditor::textColourId, Colours::black);
        version2->setColour (TextEditor::backgroundColourId, Colour (0x00000000));

        version2->setBounds (0, 721, 312, 16);

        helpButton.reset (new TextButton ("helpButton"));
        addAndMakeVisible (helpButton.get());
        helpButton->setButtonText (TRANS("Help"));
        helpButton->addListener (this);

        helpButton->setBounds (655, 3, 55, 24);

        presetButton.reset (new TextButton ("helpButton"));
        addAndMakeVisible (presetButton.get());
        presetButton->setButtonText (TRANS("Presets"));
        presetButton->addListener (this);

        presetButton->setBounds (8, 3, 55, 24);


        //[UserPreSize]
        auto buildinfo = Build::git_commit_hash + " / " + Build::build_date;
        version->setText( buildinfo.c_str(), dontSendNotification );

        sclTextAndControls = dynamic_cast<TWSTextAndControls*>( tabbedComponent->getTabContentComponent(0) );
        sclTextAndControls->textEditor->setText( processor.currentSCLString );

        kbmTextAndControls = dynamic_cast<TWSTextAndControls*>( tabbedComponent->getTabContentComponent(1) );
        kbmTextAndControls->textEditor->setText( processor.currentKBMString );

        sclTextAndControls->applyButton->setEnabled( false ); // bit of a hack
        kbmTextAndControls->applyButton->setEnabled( false ); // bit of a hack

        tuningGrid->mod->tuningUpdated( processor.tuning );

        processor.addTuningUpdatedListener( sclTextAndControls );
        processor.addTuningUpdatedListener( kbmTextAndControls );
        processor.addTuningUpdatedListener( tuningGrid.get() );
        processor.addNotesOnChangedListener( tuningGrid.get() );
        processor.addNotesOnChangedListener( sclTextAndControls );

        wheelLab->setColour (Label::backgroundColourId, getLookAndFeel().findColour (ResizableWindow::backgroundColourId));

        VCOPower->setToggleState( true, dontSendNotification );
        VCOPower->addListener(this);
        SubPower->addListener(this);
        PluckPower->addListener(this);
        DelayPower->addListener(this);
        ModWheelPower->addListener(this);
        FilterPower->addListener(this);

        presetMenu = std::make_unique<PopupMenu>();
        //[/UserPreSize]

        setSize (784, 742);


        //[Constructor] You can add your own custom stuff here..
        //[/Constructor]
        */
    }
}

impl Drop for TWSMainPanel {
    fn drop(&mut self) {
        todo!();
        /*
            //[Destructor_pre]. You can add your own custom destruction code here..
        // Clear our attachments before we destroy our components
        processor.removeTuningUpdatedListener( sclTextAndControls );
        processor.removeTuningUpdatedListener( kbmTextAndControls );
        processor.removeTuningUpdatedListener( tuningGrid.get() );
        processor.removeNotesOnChangedListener( tuningGrid.get() );
        processor.removeNotesOnChangedListener( sclTextAndControls );

        sliderAttachments.clear();
        buttonAttachments.clear();
        for( auto &p: lambdaAtttachments )
        {
            processor.parameters.getParameter(p.first.c_str())->removeListener(p.second);
            delete p.second;
        }
        lambdaAtttachments.clear();
        //[/Destructor_pre]

        version3 = nullptr;
        groupComponent7 = nullptr;
        groupComponent8 = nullptr;
        tuningGrid = nullptr;
        groupComponent3 = nullptr;
        groupComponent2 = nullptr;
        groupComponent = nullptr;
        squareMix = nullptr;
        tabbedComponent = nullptr;
        sineMix = nullptr;
        sawMix = nullptr;
        triMix = nullptr;
        AEG_A = nullptr;
        FEG_A = nullptr;
        AEG_D = nullptr;
        FEG_D = nullptr;
        AEG_S = nullptr;
        FEG_S = nullptr;
        AEG_R = nullptr;
        FEG_R = nullptr;
        groupComponent4 = nullptr;
        Filt_Cutoff = nullptr;
        Filt_Q = nullptr;
        LPFToggle = nullptr;
        HPFToggle = nullptr;
        BPFToggle = nullptr;
        groupComponent5 = nullptr;
        master_sat = nullptr;
        master_out = nullptr;
        FEG_depth = nullptr;
        uni_spread = nullptr;
        uni_count = nullptr;
        pb_up = nullptr;
        pb_down = nullptr;
        version = nullptr;
        aboutButton = nullptr;
        lfo_rate = nullptr;
        lfo_delay = nullptr;
        lfo_attack = nullptr;
        lfotritog = nullptr;
        lfosqrtog = nullptr;
        lforndtog = nullptr;
        groupComponent9 = nullptr;
        sublevel = nullptr;
        sub_oct = nullptr;
        groupComponent10 = nullptr;
        groupComponent11 = nullptr;
        delay_time = nullptr;
        pluck_atn = nullptr;
        pluck_flt = nullptr;
        pluck_init = nullptr;
        pluck_lev = nullptr;
        SubPower = nullptr;
        PluckPower = nullptr;
        DelayPower = nullptr;
        VCOPower = nullptr;
        wheelLab = nullptr;
        ModWheelPower = nullptr;
        groupComponent6 = nullptr;
        FilterPower = nullptr;
        delay_fb = nullptr;
        delay_wet = nullptr;
        delay_dry = nullptr;
        lfo_pitch = nullptr;
        lfo_sublev = nullptr;
        lfo_vcolev = nullptr;
        lfo_plucklev = nullptr;
        lfo_filter = nullptr;
        version2 = nullptr;
        helpButton = nullptr;
        presetButton = nullptr;


        //[Destructor]. You can add your own custom destruction code here..
        //[/Destructor]
        */
    }
}

impl TWSMainPanel {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            //[UserPrePaint] Add your own custom painting code here..
        //[/UserPrePaint]

        g.fillAll (Colour (0xff323e44));

        {
            int x = 0, y = 3, width = proportionOfWidth (1.0000f), height = 27;
            String text (TRANS("Tuning Workbench"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (20.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 227, y = 43, width = 22, height = 21;
            String text (TRANS("A"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 291, y = 43, width = 22, height = 21;
            String text (TRANS("D"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 355, y = 43, width = 22, height = 21;
            String text (TRANS("S"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 419, y = 43, width = 22, height = 21;
            String text (TRANS("R"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 227, y = 139, width = 22, height = 21;
            String text (TRANS("A"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 291, y = 139, width = 22, height = 21;
            String text (TRANS("D"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 355, y = 139, width = 22, height = 21;
            String text (TRANS("S"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 419, y = 139, width = 22, height = 21;
            String text (TRANS("R"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 635, y = 43, width = 22, height = 21;
            String text (TRANS("f"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 699, y = 43, width = 22, height = 21;
            String text (TRANS("q"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 635, y = 235, width = 22, height = 21;
            String text (TRANS("sat"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 699, y = 235, width = 22, height = 21;
            String text (TRANS("out"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 480, y = 45, width = 22, height = 21;
            String text (TRANS("mod"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 21, y = 44, width = 31, height = 30;
            String text (TRANS("Sine"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 21, y = 68, width = 31, height = 30;
            String text (TRANS("Sqr"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 21, y = 92, width = 31, height = 30;
            String text (TRANS("Saw"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 21, y = 116, width = 31, height = 30;
            String text (TRANS("Tri"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 516, y = 181, width = 31, height = 30;
            String text (TRANS("Dn"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 516, y = 149, width = 31, height = 30;
            String text (TRANS("Up"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 116, y = 185, width = 22, height = 21;
            String text (TRANS("oct"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 360, y = 237, width = 22, height = 21;
            String text (TRANS("del"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 296, y = 237, width = 22, height = 21;
            String text (TRANS("rate"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 432, y = 237, width = 22, height = 21;
            String text (TRANS("atk"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 496, y = 237, width = 22, height = 21;
            String text (TRANS("ptc"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 520, y = 237, width = 22, height = 21;
            String text (TRANS("vco"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 640, y = 141, width = 22, height = 21;
            String text (TRANS("time"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 704, y = 141, width = 22, height = 21;
            String text (TRANS("fb"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 21, y = 236, width = 31, height = 30;
            String text (TRANS("Flt"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 21, y = 260, width = 31, height = 30;
            String text (TRANS("Attn"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 21, y = 284, width = 31, height = 30;
            String text (TRANS("Init"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centredLeft, true);
        }

        {
            int x = 144, y = 237, width = 22, height = 21;
            String text (TRANS("lev"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 726, y = 141, width = 22, height = 21;
            String text (TRANS("dry"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 750, y = 141, width = 22, height = 21;
            String text (TRANS("wet"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 152, y = 40, width = 56, height = 30;
            String text (TRANS("Unison"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 152, y = 93, width = 56, height = 30;
            String text (TRANS("Spread"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (15.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 544, y = 237, width = 22, height = 21;
            String text (TRANS("sub"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 568, y = 237, width = 22, height = 21;
            String text (TRANS("plk"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
        }

        {
            int x = 592, y = 237, width = 22, height = 21;
            String text (TRANS("f"));
            Colour fillColour = Colours::white;
            //[UserPaintCustomArguments] Customize the painting arguments here..
            //[/UserPaintCustomArguments]
            g.setColour (fillColour);
            g.setFont (Font (12.00f, Font::plain).withTypefaceStyle ("Regular"));
            g.drawText (text, x, y, width, height,
                        Justification::centred, true);
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

        if (buttonThatWasClicked == LPFToggle.get())
        {
            //[UserButtonCode_LPFToggle] -- add your button handler code here..
            if( buttonThatWasClicked->getToggleState() )
            {
                auto ft = processor.parameters.getParameter("filter_type");
                ft->beginChangeGesture();
                ft->setValueNotifyingHost( 0.0 );
                ft->endChangeGesture();
            }
            //[/UserButtonCode_LPFToggle]
        }
        else if (buttonThatWasClicked == HPFToggle.get())
        {
            //[UserButtonCode_HPFToggle] -- add your button handler code here..
            if( buttonThatWasClicked->getToggleState() )
            {
                auto ft = processor.parameters.getParameter("filter_type");
                ft->beginChangeGesture();
                ft->setValueNotifyingHost( 0.5 );
                ft->endChangeGesture();
            }
            //[/UserButtonCode_HPFToggle]
        }
        else if (buttonThatWasClicked == BPFToggle.get())
        {
            //[UserButtonCode_BPFToggle] -- add your button handler code here..
            if( buttonThatWasClicked->getToggleState() )
            {
                auto ft = processor.parameters.getParameter("filter_type");
                ft->beginChangeGesture();
                ft->setValueNotifyingHost( 1.0 );
                ft->endChangeGesture();
            }
            //[/UserButtonCode_BPFToggle]
        }
        else if (buttonThatWasClicked == aboutButton.get())
        {
            //[UserButtonCode_aboutButton] -- add your button handler code here..
            auto te = new TWSAbout();

            DialogWindow::LaunchOptions options;
            options.content.setOwned(te);
            options.dialogTitle = "About";
            options.escapeKeyTriggersCloseButton = true;
            options.useNativeTitleBar = false;
            options.resizable = false;

            options.launchAsync();
            //[/UserButtonCode_aboutButton]
        }
        else if (buttonThatWasClicked == lfotritog.get())
        {
            //[UserButtonCode_lfotritog] -- add your button handler code here..
            if( buttonThatWasClicked->getToggleState() )
            {
                auto ft = processor.parameters.getParameter("lfo_type");
                ft->beginChangeGesture();
                ft->setValueNotifyingHost( 0.0 );
                ft->endChangeGesture();
            }

            //[/UserButtonCode_lfotritog]
        }
        else if (buttonThatWasClicked == lfosqrtog.get())
        {
            //[UserButtonCode_lfosqrtog] -- add your button handler code here..
            if( buttonThatWasClicked->getToggleState() )
            {
                auto ft = processor.parameters.getParameter("lfo_type");
                ft->beginChangeGesture();
                ft->setValueNotifyingHost( 0.5 );
                ft->endChangeGesture();
            }
            //[/UserButtonCode_lfosqrtog]
        }
        else if (buttonThatWasClicked == lforndtog.get())
        {
            //[UserButtonCode_lforndtog] -- add your button handler code here..
            if( buttonThatWasClicked->getToggleState() )
            {
                auto ft = processor.parameters.getParameter("lfo_type");
                ft->beginChangeGesture();
                ft->setValueNotifyingHost( 1.0 );
                ft->endChangeGesture();
            }
            //[/UserButtonCode_lforndtog]
        }
        else if (buttonThatWasClicked == helpButton.get())
        {
            //[UserButtonCode_helpButton] -- add your button handler code here..
            //[/UserButtonCode_helpButton]
        }
        else if (buttonThatWasClicked == presetButton.get())
        {
            //[UserButtonCode_presetButton] -- add your button handler code here..
            doPresetMenu();
            //[/UserButtonCode_presetButton]
        }

        //[UserbuttonClicked_Post]
        auto stv = [this, &buttonThatWasClicked](const char* lb) {
                       auto ft = this->processor.parameters.getParameter(lb);
                       ft->beginChangeGesture();
                       ft->setValueNotifyingHost(buttonThatWasClicked->getToggleState() ? 1 : 0 );
                       ft->endChangeGesture();
                   };
        if (buttonThatWasClicked == VCOPower.get() )
        {
            stv( "vco_on" );
        }
        if (buttonThatWasClicked == SubPower.get() )
        {
            stv( "sub_on" );
        }
        if (buttonThatWasClicked == PluckPower.get() )
        {
            stv( "pluck_on" );
        }
        if (buttonThatWasClicked == DelayPower.get() )
        {
            stv( "delay_on" );
        }
        if (buttonThatWasClicked == ModWheelPower.get() )
        {
            stv( "modwheel_on" );
        }
        if (buttonThatWasClicked == FilterPower.get() )
        {
            stv( "filter_on" );
        }

        //[/UserbuttonClicked_Post]
        */
    }
    
    pub fn slider_value_changed(&mut self, slider_that_was_moved: *mut Slider)  {
        
        todo!();
        /*
            //[UsersliderValueChanged_Pre]
        //[/UsersliderValueChanged_Pre]

        if (sliderThatWasMoved == delay_fb.get())
        {
            //[UserSliderCode_delay_fb] -- add your slider handling code here..
            //[/UserSliderCode_delay_fb]
        }
        else if (sliderThatWasMoved == delay_wet.get())
        {
            //[UserSliderCode_delay_wet] -- add your slider handling code here..
            //[/UserSliderCode_delay_wet]
        }
        else if (sliderThatWasMoved == delay_dry.get())
        {
            //[UserSliderCode_delay_dry] -- add your slider handling code here..
            //[/UserSliderCode_delay_dry]
        }
        else if (sliderThatWasMoved == lfo_pitch.get())
        {
            //[UserSliderCode_lfo_pitch] -- add your slider handling code here..
            //[/UserSliderCode_lfo_pitch]
        }
        else if (sliderThatWasMoved == lfo_sublev.get())
        {
            //[UserSliderCode_lfo_sublev] -- add your slider handling code here..
            //[/UserSliderCode_lfo_sublev]
        }
        else if (sliderThatWasMoved == lfo_vcolev.get())
        {
            //[UserSliderCode_lfo_vcolev] -- add your slider handling code here..
            //[/UserSliderCode_lfo_vcolev]
        }
        else if (sliderThatWasMoved == lfo_plucklev.get())
        {
            //[UserSliderCode_lfo_plucklev] -- add your slider handling code here..
            //[/UserSliderCode_lfo_plucklev]
        }
        else if (sliderThatWasMoved == lfo_filter.get())
        {
            //[UserSliderCode_lfo_filter] -- add your slider handling code here..
            //[/UserSliderCode_lfo_filter]
        }

        //[UsersliderValueChanged_Post]
        //[/UsersliderValueChanged_Post]
        */
    }
    
    pub fn files_dropped(&mut self, 
        filenames: &StringArray,
        mousex:    i32,
        mousey:    i32)  {
        
        todo!();
        /*
            //[UserCode_filesDropped] -- Add your code here...
        if( filenames.size() != 1 ) return;
        File f( filenames[0] );
        if( f.hasFileExtension( ".scl" ) )
        {
            auto s = f.loadFileAsString();
            processor.setSCL( s );
            tabbedComponent->setCurrentTabIndex( 0, false );
        }
        if( f.hasFileExtension( ".kbm" ) )
        {
            auto s = f.loadFileAsString();
            processor.setKBM( s );
            tabbedComponent->setCurrentTabIndex( 1, false );
        }
        if( f.hasFileExtension( ".twsxml" ) )
        {
            auto s = f.loadFileAsString();
            MemoryBlock b;
            f.loadFileAsData( b );
            processor.setStateInformation( b.getData(), (int)b.getSize() );
        }


        //[/UserCode_filesDropped]
        */
    }

    pub fn connect_value_tree_state(&mut self, t: &mut AudioProcessorValueTreeState)  {
        
        todo!();
        /*
            auto s = [this, &t](const char* lb, std::unique_ptr<Slider> &sll )
                     {
                         sliderAttachments.push_back(std::make_unique<SliderAttachment>( t, lb, *(sll.get() ) ) );
                     };

        auto pb = [this, &t](const char *lb, Button *tb )
                      {
                          buttonAttachments.push_back(std::make_unique<ButtonAttachment>( t, lb, *( tb ) ) );
                      };
        
        s("sinLevel", sineMix);
        s("squareLevel", squareMix);
        s("sawLevel", sawMix);
        s("triLevel", triMix);

        s("uni_count", uni_count );
        s("uni_spread", uni_spread );

        s( "pb_down", pb_down );
        s( "pb_up", pb_up );

        s( "amp_attack", AEG_A );
        s( "amp_decay", AEG_D );
        s( "amp_sustain", AEG_S );
        s( "amp_release", AEG_R );

        s( "filter_attack", FEG_A );
        s( "filter_decay", FEG_D );
        s( "filter_sustain", FEG_S );
        s( "filter_release", FEG_R );
        s( "filter_depth", FEG_depth );

        s( "filter_cutoff", Filt_Cutoff );
        s( "filter_resonance", Filt_Q );
        s( "master_sat", master_sat );
        s( "master_level", master_out );

        s( "subosc_level", sublevel );
        s( "subosc_oct", sub_oct );
        s( "pluck_flt", pluck_flt );
        s( "pluck_atn", pluck_atn );
        s( "pluck_init", pluck_init );
        s( "pluck_lev", pluck_lev );

        s( "lfo_rate", lfo_rate );
        s( "lfo_delay", lfo_delay );
        s( "lfo_attack", lfo_attack );
        s( "lfo_pitch", lfo_pitch );
        s( "lfo_filter", lfo_filter );
        s( "lfo_plucklev", lfo_plucklev );
        s( "lfo_sublev", lfo_sublev );
        s( "lfo_vcolev", lfo_vcolev );

        s( "delay_fb", delay_fb );
        s( "delay_wet", delay_wet );
        s( "delay_dry", delay_dry );
        s( "delay_time", delay_time );

        pb( "vco_on", VCOPower.get() );
        pb( "sub_on", SubPower.get() );
        pb( "pluck_on", PluckPower.get() );
        pb( "delay_on", DelayPower.get() );
        pb( "modwheel_on", ModWheelPower.get() );
        pb( "filter_on", FilterPower.get() );
        
        /*
        ** Hook up the filter tri-state and LFO tri-state
        */
        {
            auto l = new TWSLambdaParamListener();
            l->lfunc = [this](int i, float v)
                           {
                               this->LPFToggle->setToggleState( v == 0, dontSendNotification );
                               this->HPFToggle->setToggleState( v == 0.5, dontSendNotification );
                               this->BPFToggle->setToggleState( v == 1, dontSendNotification );
                           };
            t.getParameter("filter_type")->addListener(l);
            l->lfunc(0,t.getParameter("filter_type")->getValue());
            lambdaAtttachments.push_back(std::make_pair(std::string("filter_type"), l));
        }
        {
            auto l = new TWSLambdaParamListener();
            l->lfunc = [this](int i, float v)
                           {
                               this->lfotritog->setToggleState( v == 0, dontSendNotification );
                               this->lfosqrtog->setToggleState( v == 0.5, dontSendNotification );
                               this->lforndtog->setToggleState( v == 1, dontSendNotification );
                           };
            t.getParameter("lfo_type")->addListener(l);
            l->lfunc(0,t.getParameter("lfo_type")->getValue());
            lambdaAtttachments.push_back(std::make_pair(std::string("lfo_type"), l));
        }
        */
    }
}

impl TWSPowerToggle {
    
    pub fn paint_button(&mut self, 
        g:  &mut Graphics,
        hl: bool,
        dn: bool)  {
        
        todo!();
        /*
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        if( getToggleState() )
        {
            g.setColour( findColour( Slider::thumbColourId ) );
            g.fillEllipse( 2, 3, 12, 12 );
            g.setColour( findColour( Label::textColourId ) );
            g.drawEllipse( 2, 3, 12, 11, 1 );
            g.fillRect( 7, 2, 2, 6 );
        }
        else
        {
            g.setColour( findColour( GroupComponent::outlineColourId ) );
            g.drawEllipse( 2, 3, 12, 12, 1 );
            g.fillRect( 7, 3, 2, 6 );
        }
        */
    }
}


impl TWSMainPanel {
    
    pub fn do_preset_menu(&mut self)  {
        
        todo!();
        /*
            presetMenu->clear();
        for( int i=0; i<processor.factoryPresets.size(); ++i )
        {
            auto t = processor.factoryPresets[i];
            auto n = std::get<0>(t);
            auto b = std::get<1>(t);
            auto s = std::get<2>(t);
            if( b == 0 || s == 0 )
                presetMenu->addSectionHeader( n );
            else
                presetMenu->addItem( i, n );
        }
        presetMenu->addSeparator();

        presetMenu->addItem( 10000, "Save Preset As..." );
        presetMenu->addItem( 10001, "Load Preset From..." );

        auto ri = presetButton->getScreenBounds();
        // ri.setY( ri.getY() + presetButton->getHeight() );

        auto res = presetMenu->showAt( ri );
        switch( res )
        {
        case 10000:
        {
            FileChooser fc( "Save Preset As..", File(), "*.twsxml" );
            if( fc.browseForFileToSave(true) )
            {
                auto f = fc.getResult();
                MemoryBlock b;
                processor.getStateInformation(b);
                if( ! f.replaceWithData( b.getData(),b.getSize() ) )
                {
                    AlertWindow::showMessageBoxAsync( AlertWindow::AlertIconType::WarningIcon,
                                                      "Error saving preset",
                                                      "An unknown error occured streaming data to file",
                                                      "OK" );
                }
            }
            break;
        }
        case 10001:
        {
            FileChooser fc( "Load Preset From...", File(), "*.twsxml" );
            if( fc.browseForFileToOpen() )
            {
                auto f = fc.getResult();
                MemoryBlock b;
                f.loadFileAsData( b );
                processor.setStateInformation( b.getData(), (int)b.getSize() );
            }
            break;
        }
        default:
        {
            auto t = processor.factoryPresets[res];
            auto b = std::get<1>(t);
            auto s = std::get<2>(t);
            processor.setStateInformation( b, (int)s );

            break;
        }
        }
        */
    }
}
