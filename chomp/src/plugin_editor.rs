crate::ix!();

pub struct TuningworkbenchsynthAudioProcessorEditor {

    main_panel: TWSMainPanel,

    /**
      | This reference is provided as a quick
      | way for your editor to access the processor
      | object that created it.
      |
      */
    processor:  &mut TuningworkbenchsynthAudioProcessor,

    parameters: &mut AudioProcessorValueTreeState,
}

impl AudioProcessorEditor for TuningworkbenchsynthAudioProcessorEditor {

}

impl TuningworkbenchsynthAudioProcessorEditor {
    
    pub fn new(
        p:   &mut TuningworkbenchsynthAudioProcessor,
        vts: &mut AudioProcessorValueTreeState) -> Self {
    
        todo!();
        /*
        : audio_processor_editor(&p),
        : main_panel(p ),
        : processor(p),
        : parameters(vts ),

            // Make sure that before the constructor has finished, you've set the
        // editor's size to whatever you need it to be.
        setSize (784, 742);

        // Do some look and feel magic
        mainPanel.connectValueTreeState( vts );
        addAndMakeVisible(mainPanel);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // (Our component is opaque, so we must completely fill the background with a solid colour)
        g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // This is generally where you'll want to lay out the positions of any
        // subcomponents in your editor..
        */
    }
}


