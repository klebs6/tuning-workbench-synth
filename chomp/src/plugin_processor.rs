crate::ix!();

pub struct TWSSynthesiser {
    processor:     &mut TuningworkbenchsynthAudioProcessor,
    linel:         Vec<f32>,
    liner:         Vec<f32>,
    delay_pos:     usize,
    delay_size:    usize,
    last_delay_on: bool, // default = true
    delayt:        SmoothedValue<f32>,
    delayfb:       SmoothedValue<f32>,
    delay_wet:     SmoothedValue<f32>,
    delay_dry:     SmoothedValue<f32>,
}

impl Synthesizer for TWSSynthesiser {

}

impl TWSSynthesiser {
    
    pub fn new(p: &mut TuningworkbenchsynthAudioProcessor) -> Self {
    
        todo!();
        /*


        
        */
    }
    
    pub fn render_voices(&mut self, 
        b: &mut AudioBuffer<f32>,
        s: i32,
        n: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn handle_controller(&mut self, 
        chan: i32,
        cont: i32,
        val:  i32)  {
        
        todo!();
        /*
        
        */
    }
}

impl AudioProcessor for TuningworkbenchsynthAudioProcessor {}

pub struct TuningworkbenchsynthAudioProcessor {
    tuning_listeners:   HashSet<*mut TuningUpdatedListener>,
    note_listeners:     HashSet<*mut NotesOnChangedListener>,
    tuning:             Tuning,
    current_scl_string: String, // default = ""
    current_kbm_string: String, // default = ""
    synth:              TWSSynthesiser,
    parameters:         AudioProcessorValueTreeState,
    
    /*
    atomic<float> *sinLevel, *squareLevel, *sawLevel, *triLevel;
    std::atomic<float> *uni_count; // as float
    std::atomic<float> *uni_spread;

    std::atomic<float> *pb_down, *pb_up; // as float

    std::atomic<float> *amp_attack, *amp_decay, *amp_sustain, *amp_release;
    std::atomic<float> *filter_attack, *filter_decay, *filter_sustain, *filter_release, *filter_depth;

    std::atomic<float> *filter_type;
    std::atomic<float> *filter_cutoff, *filter_resonance;

    std::atomic<float> *master_sat, *master_level;

    std::atomic<float> *subosc_level, *subosc_oct;
    std::atomic<float> *pluck_flt, *pluck_atn, *pluck_init, *pluck_lev;
    std::atomic<float> *delay_time, *delay_fb, *delay_wet, *delay_dry;

    std::atomic<float> *lfo_type, *lfo_rate, *lfo_delay, *lfo_attack, *lfo_pitch, *lfo_filter, *lfo_vcolev, *lfo_sublev, *lfo_plucklev;

    std::atomic<float> *vco_on, *sub_on, *pluck_on, *delay_on, *modwheel_on, *filter_on;
    */

    factory_presets: Vec<(String,*const u8,usize)>,
}

impl TuningworkbenchsynthAudioProcessor {
    
    pub fn prepare_to_play(&mut self, 
        sample_rate:       Option<f64>,
        samples_per_block: i32)  {
        let sample_rate: f64 = sample_rate.unwrap_or(44000.0);

        todo!();
        /*
        
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_buses_layout_supported(&self, layouts: &BusesLayout) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn process_block(&mut self, 
        _0: &mut AudioBuffer<f32>,
        _1: &mut MidiBuffer)  {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn is_midi_effect(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn set_current_program(&mut self, index: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn get_program_name(&mut self, index: i32) -> String {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn change_program_name(&mut self, 
        index:    i32,
        new_name: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
        
        */
    }

    // == tuning support ==
    pub fn add_tuning_updated_listener(&mut self, l: *mut TuningUpdatedListener)  {
        
        todo!();
        /*
            tuningListeners.insert( l );
        */
    }
    
    pub fn remove_tuning_updated_listener(&mut self, l: *mut TuningUpdatedListener)  {
        
        todo!();
        /*
            tuningListeners.erase( l );
        */
    }
    
    pub fn set_scl(&mut self, 
        SCL:    String,
        retune: bool)  {
        let retune: bool = retune.unwrap_or(true );

        todo!();
        /*
        
        */
    }
    
    
    pub fn set_kbm(&mut self, 
        KBM:    String,
        retune: bool)  {
        let retune: bool = retune.unwrap_or(true );

        todo!();
        /*
        
        */
    }
    
    pub fn reset_scl_to_standard(&mut self)  {
        
        todo!();
        /*
            auto s = Tunings::evenTemperament12NoteScale();
            setSCL( s.rawText );
        */
    }
    
    pub fn reset_kbm_to_standard(&mut self)  {
        
        todo!();
        /*
            setKBM( "" );
        */
    }
    
    pub fn retune(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn add_notes_on_changed_listener(&mut self, l: *mut NotesOnChangedListener)  {
        
        todo!();
        /*
            noteListeners.insert( l );
        */
    }
    
    pub fn remove_notes_on_changed_listener(&mut self, l: *mut NotesOnChangedListener)  {
        
        todo!();
        /*
            noteListeners.erase( l );
        */
    }
    
    pub fn envelope_time_param(&mut self, 
        tag:  String,
        name: String,
        init: f32,
        min:  f32,
        max:  f32) -> Box<AudioParameterFloat> {

        let min: f32 = min.unwrap_or(0);
        let max: f32 = max.unwrap_or(10 );

        todo!();
        /*
            return std::make_unique<AudioParameterFloat>( tag, name,
                                                              NormalisableRange<float>(
                                                                  0, 10,
                                                                  [](float start, float end, float value ) -> float {
                                                                      // basically we want ( e^v/e^1 ) * end
                                                                      auto r = ( exp( value ) - 1 ) / ( exp(1) - 1 )  * end;
                                                                      return r;
                                                                  },
                                                                  [](float start, float end, float value ) -> float {
                                                                      // r = (e^v-1)/(e^1-1)*end
                                                                      // r / end * (e^1-1) + 1 = e^v;
                                                                      auto r = log( value / end * ( exp(1) - 1 ) + 1 );
                                                                      return r;
                                                                  } ),
                                                                 init,
                                                                 "", AudioProcessorParameter::genericParameter,
                                                                 [](float v, int length) -> String
                                                                     {
                                                                         String asText(v,2);
                                                                         return ( length > 0 ? asText.substring( 0, length ) : asText );
                                                                     },
                                                                 [](const String& text ) -> float { return text.getFloatValue(); }
                    );
        */
    }
    
    pub fn exp_range_param(&mut self, 
        tag:  String,
        name: String,
        low:  f32,
        high: f32,
        init: f32) -> Box<AudioParameterFloat> {
        
        todo!();
        /*
            return std::make_unique<AudioParameterFloat>( tag, name,
                                                              NormalisableRange<float>(
                                                                  low, high,
                                                                  [](float start, float end, float value ) -> float {
                                                                         // So we want 2^x flavor here
                                                                         // 2^0 + b = 10 -> b = 10
                                                                         // 2^a + b = 20000 - a = log_2( 19000 )
                                                                         auto b = start;
                                                                         auto a = log(end-start)/log(2);
                                                                         auto r = pow( 2.0, a * value ) + b;
                                                                         
                                                                         return r;
                                                                     },
                                                                     [](float start, float end, float value ) -> float {
                                                                         // 2^ar + b = v
                                                                         // 2^ar = v - b;
                                                                         // r = log2(v-b)/a
                                                                         // a = log2(end-start)
                                                                         // r = log2(v-start)/log2(end-start)
                                                                         auto r = log(value - start)/log(end - start);
                                                                         return r;
                                                                     } ),
                                                                 init,
                                                                 "", AudioProcessorParameter::genericParameter,
                                                                 [](float v, int length) -> String
                                                                     {
                                                                         String asText(v,2);
                                                                         return ( length > 0 ? asText.substring( 0, length ) : asText );
                                                                     },
                                                                 [](const String& text ) -> float { return text.getFloatValue(); }
                    );
        */
    }
}


impl TuningworkbenchsynthAudioProcessor {
    
    pub fn new() -> Self {
    
        todo!();
        /*


            : AudioProcessor (BusesProperties()
                          .withOutput ("Output", AudioChannelSet::stereo(), true)
            ),
          synth( *this ),
          parameters( *this,
                      nullptr,
                      Identifier( "tuning-workbench-synthesiser" ),
                      {
                          std::make_unique<AudioParameterFloat>( "sinLevel",
                                                                 "Sin Osc Level",
                                                                 0.0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterFloat>( "squareLevel",
                                                                 "Square Osc Level",
                                                                 0.0, 1.0, 0.2 ),
                          std::make_unique<AudioParameterFloat>( "sawLevel",
                                                                 "Saw Osc Level",
                                                                 0.0, 1.0, 0.8 ),
                          std::make_unique<AudioParameterFloat>( "triLevel",
                                                                 "Tri Osc Level",
                                                                 0.0, 1.0, 0.3 ),
                          std::make_unique<AudioParameterInt>( "uni_count",
                                                               "Unison Count",
                                                               1, 10, 3 ),
                          std::make_unique<AudioParameterFloat>( "uni_spread",
                                                                 "Unison Spread",
                                                                 1, 100, 4 ),

                          std::make_unique<AudioParameterInt>( "pb_down",
                                                               "Pitch Bend Down",
                                                               1, 12, 2 ),
                          std::make_unique<AudioParameterInt>( "pb_up",
                                                               "Pitch Bend",
                                                               1, 12, 2 ),

                          envelopeTimeParam( "amp_attack", "Amp Attack", 0.02 ),
                          envelopeTimeParam( "amp_decay", "Amp Decay", 0.1 ),
                          std::make_unique<AudioParameterFloat>( "amp_sustain",
                                                                "Amp Sustain",
                                                                0, 1.0, 0.7 ),
                          envelopeTimeParam( "amp_release", "Amp Release", 0.1 ),

                          envelopeTimeParam( "filter_attack", "Filter Attack", 0.1 ),
                          envelopeTimeParam( "filter_decay", "Filter Decay", 0.1 ),
                          std::make_unique<AudioParameterFloat>( "filter_sustain",
                                                                "Filter Sustain",
                                                                0, 1.0, 0.5 ),
                          envelopeTimeParam( "filter_release", "Filter Release", 0.1 ),
                          std::make_unique<AudioParameterFloat>( "filter_depth",
                                                                 "Filter ModulationDepth",
                                                                 -1.0, 1.0, 0.0 ),

                          std::make_unique<AudioParameterInt>( "filter_type",
                                                               "Filter Type",
                                                               1,3,1 ),
                          expRangeParam( "filter_cutoff", "Filter Cutoff", 40, 18000, 880 ),
                          std::make_unique<AudioParameterFloat>( "filter_resonance",
                                                                 "Filter Resonance",
                                                                 0, 1.0, 0.7 ),

                          std::make_unique<AudioParameterFloat>( "master_sat",
                                                                 "Master Saturation",
                                                                 0, 7.0, 0 ),
                          std::make_unique<AudioParameterFloat>( "master_level",
                                                                 "Master Level",
                                                                 0, 1.0, 1.0 ),

                          std::make_unique<AudioParameterFloat>( "subosc_level",
                                                                 "SubOSC Level",
                                                                 0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterInt>( "subosc_oct",
                                                               "SubOSC Octave",
                                                               -3, -1, -1 ),

                          std::make_unique<AudioParameterFloat>( "pluck_flt",
                                                                 "Pluck Flt",
                                                                 0.05, 0.95, 0.6 ),
                          std::make_unique<AudioParameterFloat>( "pluck_atn",
                                                                 "Pluck Atten",
                                                                 0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterInt>( "pluck_init",
                                                              "Pluck Init",
                                                              0, 5.0, 0.0 ),
                          std::make_unique<AudioParameterFloat>( "pluck_lev",
                                                                 "Pluck Level",
                                                                 0, 1.0, 1.0 ),

                          envelopeTimeParam( "delay_time", "Delay Time", 0.5, 0.05, MAX_DELAY_TIME ),
                          std::make_unique<AudioParameterFloat>( "delay_fb",
                                                                 "Delay Feedback",
                                                                 0., 1.0, 0.1 ),
                          std::make_unique<AudioParameterFloat>( "delay_wet",
                                                                 "Delay Wet Level",
                                                                 0., 1.0, 0.4 ),
                          std::make_unique<AudioParameterFloat>( "delay_dry",
                                                                 "Delay Dry Level",
                                                                 0., 1.0, 1.0 ),


                          std::make_unique<AudioParameterInt>( "lfo_type",
                                                              "LFO Type",
                                                              0, 2.0, 0.0 ),
                          std::make_unique<AudioParameterFloat>( "lfo_rate",
                                                                 "LFO Rate",
                                                                 0.0, 12.0, 3.0 ),
                          envelopeTimeParam( "lfo_delay", "LFO Delay", 0.0 ),
                          envelopeTimeParam( "lfo_attack", "LFO Attack", 0.0 ),
                          std::make_unique<AudioParameterFloat>( "lfo_pitch",
                                                                 "LFO Pitch",
                                                                 0.0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterFloat>( "lfo_vcolev",
                                                                 "LFO VCO Level",
                                                                 0.0, 1.0, 0.1 ),
                          std::make_unique<AudioParameterFloat>( "lfo_sublev",
                                                                 "LFO VCO Level",
                                                                 0.0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterFloat>( "lfo_plucklev",
                                                                 "LFO VCO Level",
                                                                 0.0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterFloat>( "lfo_filter",
                                                                 "LFO Filter",
                                                                 0.0, 10.0, 0.0 ),

                          std::make_unique<AudioParameterInt>( "vco_on",
                                                               "VCO On",
                                                               0, 1.0, 1.0 ),
                          std::make_unique<AudioParameterInt>( "sub_on",
                                                               "SUB On",
                                                               0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterInt>( "pluck_on",
                                                               "PLUCK On",
                                                               0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterInt>( "delay_on",
                                                               "Delay On",
                                                               0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterInt>( "modwheel_on",
                                                               "ModWheel On",
                                                               0, 1.0, 0.0 ),
                          std::make_unique<AudioParameterInt>( "filter_on",
                                                               "Filter On",
                                                               0, 1.0, 1.0 )

                      } )



        for( int i=0; i<32; ++i )
        {
            auto v = new TWSVoice(this);
            addTuningUpdatedListener( v );
            synth.addVoice( v );
        }

        synth.addSound( new TWSSound( this ) );

    #define SP(x) x = parameters.getRawParameterValue( #x )
        
        SP(sinLevel);
        SP(squareLevel);
        SP(sawLevel);
        SP(triLevel);

        SP(uni_count);
        SP(uni_spread);
        
        SP(amp_attack);
        SP(amp_decay);
        SP(amp_sustain);
        SP(amp_release);

        SP(filter_attack);
        SP(filter_decay);
        SP(filter_sustain);
        SP(filter_release);
        SP(filter_depth);

        SP(filter_type);
        SP(filter_cutoff);
        SP(filter_resonance);

        SP(master_sat);
        SP(master_level);

        SP(pb_down);
        SP(pb_up);

        SP(subosc_level);
        SP(subosc_oct);
        SP(pluck_flt);
        SP(pluck_atn);
        SP(pluck_init);
        SP(pluck_lev);

        SP(delay_time);
        SP(delay_fb);
        SP(delay_wet);
        SP(delay_dry);

        SP(lfo_type);
        SP(lfo_rate);
        SP(lfo_delay);
        SP(lfo_attack);
        SP(lfo_pitch);
        SP(lfo_vcolev);
        SP(lfo_sublev);
        SP(lfo_plucklev);
        SP(lfo_filter);

        SP(vco_on);
        SP(sub_on);
        SP(pluck_on);
        SP(delay_on);
        SP(modwheel_on);
        SP(filter_on);
        
        auto initS = Tunings::evenTemperament12NoteScale();
        setSCL( initS.rawText, false );
        setKBM( "", false );
        retune();

        // And finally set up the factory patches.
        auto mt = []( std::string a, const char* b, size_t c ) {
                      return std::make_tuple(a,b,c);
                  };
        factoryPresets.push_back( mt( "Init", 0, 0 ) );              
        factoryPresets.push_back( mt( "Init Sine", BinaryData::INIT_Sine_twsxml, BinaryData::INIT_Sine_twsxmlSize ) );
        factoryPresets.push_back( mt( "Init Square", BinaryData::INIT_Square_twsxml, BinaryData::INIT_Square_twsxmlSize ) );
        factoryPresets.push_back( mt( "Init Saw", BinaryData::INIT_Saw_twsxml, BinaryData::INIT_Saw_twsxmlSize ) );
        factoryPresets.push_back( mt( "TWS Init ", BinaryData::TWS_Init_twsxml, BinaryData::TWS_Init_twsxmlSize ) );
        factoryPresets.push_back( mt( "Pads", 0, 0 ) );             
        factoryPresets.push_back( mt( "Discovery", BinaryData::Discovery_twsxml, BinaryData::Discovery_twsxmlSize ) );              
        factoryPresets.push_back( mt( "Facets 15", BinaryData::Facets_15_twsxml, BinaryData::Just_Frippertonics_twsxmlSize ) );              
        factoryPresets.push_back( mt( "On Whole Tones", BinaryData::On_Whole_Tones_twsxml, BinaryData::On_Whole_Tones_twsxmlSize ) );              
        factoryPresets.push_back( mt( "Swell Pulse", BinaryData::Swell_Pulse_Pad_twsxml, BinaryData::Swell_Pulse_Pad_twsxmlSize ) );              
        factoryPresets.push_back( mt( "Xenarctica", BinaryData::Xenarctica_twsxml, BinaryData::Xenarctica_twsxmlSize ) );              
        factoryPresets.push_back( mt( "Keys and Mallets", 0, 0 ) );
        factoryPresets.push_back( mt( "Marimba", BinaryData::Marimba_twsxml, BinaryData::Marimba_twsxmlSize ) );
        factoryPresets.push_back( mt( "Space Clav", BinaryData::Space_Clav_twsxml, BinaryData::Space_Clav_twsxmlSize ) );
        factoryPresets.push_back( mt( "Strings", 0, 0 ) );
        factoryPresets.push_back( mt( "Fibonacci Plucks", BinaryData::Fibonacci_Plucks_twsxml, BinaryData::Fibonacci_Plucks_twsxmlSize ) );    
        factoryPresets.push_back( mt( "Pluckedelica", BinaryData::Pluckedelica_twsxml, BinaryData::Pluckedelica_twsxmlSize ) );
        factoryPresets.push_back( mt( "Struck String", BinaryData::Struck_String_twsxml, BinaryData::Struck_String_twsxmlSize ) );
        factoryPresets.push_back( mt( "Swarmandal", BinaryData::Swarm_Andal_twsxml, BinaryData::Swarm_Andal_twsxmlSize ) );    
        factoryPresets.push_back( mt( "Winds", 0, 0 ) );
        factoryPresets.push_back( mt( "Bamboo Flute", BinaryData::Bamboo_Flute_twsxml, BinaryData::Bamboo_Flute_twsxmlSize ) );
        factoryPresets.push_back( mt( "Brass Lead", BinaryData::Brass_Lead_twsxml, BinaryData::Brass_Lead_twsxmlSize ) );    
        factoryPresets.push_back( mt( "Lusheng Flute", BinaryData::Lusheng_Flute_twsxml, BinaryData::Lusheng_Flute_twsxmlSize ) );
        */
    }
}

impl Drop for TuningworkbenchsynthAudioProcessor {
    fn drop(&mut self) {
        todo!();
        /*
            for( int i=0; i<synth.getNumVoices(); ++i )
        {
            auto tl = dynamic_cast<TuningUpdatedListener*>( synth.getVoice(i) );
            if( tl )
                removeTuningUpdatedListener( tl );
        }
        */
    }
}

impl TuningworkbenchsynthAudioProcessor {
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return JucePlugin_Name;
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    
    pub fn is_midi_effect(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    
    pub fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            return 0.0;
        */
    }
    
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return 1;   // NB: some hosts don't cope very well if you tell them there are 0 programs,
                    // so this should be at least 1, even if you're not really implementing programs.
        */
    }
    
    
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    
    pub fn set_current_program(&mut self, index: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn get_program_name(&mut self, index: i32) -> String {
        
        todo!();
        /*
            return {};
        */
    }
    
    
    pub fn change_program_name(&mut self, 
        index:    i32,
        new_name: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    
    pub fn prepare_to_play(&mut self, 
        sample_rate:       f64,
        samples_per_block: i32)  {
        
        todo!();
        /*
            synth.setCurrentPlaybackSampleRate( sampleRate );
        */
    }
    
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
            // When playback stops, you can use this as an opportunity to free up any
        // spare memory, etc.
        */
    }
    
    
    pub fn is_buses_layout_supported(&self, layouts: &BusesLayout) -> bool {
        
        todo!();
        /*
            // This is the place where you check if the layout is supported.
        // In this template code we only support mono or stereo.
        if (layouts.getMainOutputChannelSet() != AudioChannelSet::mono()
            && layouts.getMainOutputChannelSet() != AudioChannelSet::stereo())
            return false;

        // This checks if the input layout matches the output layout
        if (layouts.getMainOutputChannelSet() != layouts.getMainInputChannelSet())
            return false;

        return true;
        */
    }
    
    
    pub fn process_block(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer)  {
        
        todo!();
        /*
            if( noteListeners.size() > 0 )
        {
            bool noteTog = false;
            MidiBuffer::Iterator midiIterator (midiMessages);
            midiIterator.setNextSamplePosition (0);
            int midiEventPos;
            MidiMessage m;

            int ons[127], offs[127];
            int onp=0, offp=0;

            while( midiIterator.getNextEvent(m, midiEventPos ) )
            {
                if( m.isNoteOn() )
                {
                    ons[onp++] = m.getNoteNumber();
                    noteTog = true;
                }
                else if( m.isNoteOff() )
                {
                    offs[offp++] = m.getNoteNumber();
                    noteTog = true;
                }
            }

            if( noteTog )
            {
                /*
                ** We have to have at least one oscillator on (although this is a rare case)
                */
                if( ! ( *vco_on || *pluck_on || *sub_on ) )
                {
                    *vco_on = 1.0; // this will get re-written in the async update
                    auto vo = parameters.getParameter( "vco_on" );
                    vo->beginChangeGesture();
                    vo->setValueNotifyingHost( 1.0 );
                    vo->endChangeGesture();
                }
                for( auto l : noteListeners )
                {
                    for( int i=0; i<onp; ++i )
                        l->noteOn( ons[i] );
                    for( int i=0; i<offp; ++i )
                        l->noteOff( offs[i] );
                }
            }
        }

        synth.renderNextBlock(buffer, midiMessages, 0, buffer.getNumSamples() );
        */
    }
    
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true; // (change this to false if you choose to not supply an editor)
        */
    }
    
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new TuningworkbenchsynthAudioProcessorEditor (*this, parameters);
        */
    }
    
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            auto state = parameters.copyState();
        std::unique_ptr<XmlElement> xml (state.createXml());

        xml->deleteAllChildElementsWithTagName( "tuningState" );
        
        auto txml = xml->createNewChildElement( "tuningState" );
        auto sclx = txml->createNewChildElement( "scl" );
        sclx->addTextElement(currentSCLString);
        auto kbmx = txml->createNewChildElement( "kbm" );
        kbmx->addTextElement(currentKBMString);

        copyXmlToBinary (*xml, destData);
        */
    }
    
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            std::unique_ptr<XmlElement> xmlState (getXmlFromBinary (data, sizeInBytes));

        if (xmlState.get() != nullptr)
        {
            if (xmlState->hasTagName (parameters.state.getType()))
                parameters.replaceState (ValueTree::fromXml (*xmlState));


            auto txml = xmlState->getChildByName( "tuningState" );
            if( txml )
            {
                bool needsRT = false;

                auto sclx = txml->getChildByName( "scl" );
                auto kbmx = txml->getChildByName( "kbm" );
                std::string s = "";

                if( sclx && sclx->getFirstChildElement() && sclx->getFirstChildElement()->isTextElement() )
                {
                    s = sclx->getFirstChildElement()->getText().toStdString();
                    if( s.size() > 1 )
                    {
                        needsRT = true;
                        setSCL( s, false );
                    }
                }
                
                std::string k = "";
                if( kbmx && kbmx->getFirstChildElement() && kbmx->getFirstChildElement()->isTextElement() )
                {
                    k = kbmx->getFirstChildElement()->getText().toStdString();
                    if( k.size() > 1 )
                    {
                        needsRT = true;
                        setKBM( k, false );
                    }
                }

                if( needsRT )
                {
                    retune();
                }
            }
        }
        */
    }
    
    
    pub fn set_scl(&mut self, 
        SCL:     String,
        dretune: bool)  {
        
        todo!();
        /*
            currentSCLString = SCL;
        if( dretune )
            retune();
        */
    }
    
    
    pub fn set_kbm(&mut self, 
        KBM:     String,
        dretune: bool)  {
        
        todo!();
        /*
            currentKBMString = KBM;
        if( dretune )
            retune();
        */
    }
    
    
    pub fn retune(&mut self)  {
        
        todo!();
        /*
            bool sclGood = false;
        try
        {
            auto s = Tunings::parseSCLData( currentSCLString.toStdString() );
            sclGood = true;
            auto k = Tunings::KeyboardMapping();
            if( currentKBMString.length() > 0 )
            {
                k = Tunings::parseKBMData( currentKBMString.toStdString() );
            }
            tuning = Tunings::Tuning( s, k );
            for( auto l : tuningListeners )
            {
                l->tuningUpdated( tuning );
            }
        }
        catch( Tunings::TuningError &e )
        {
            AlertWindow::showMessageBoxAsync( AlertWindow::AlertIconType::WarningIcon,
                                              "Error Applying Tuning",
                                              e.what(),
                                              "OK" );
        }
        */
    }
}

///==============================================================================
/// This creates new instances of the plugin..
///
#[JUCE_CALLTYPE]
pub fn create_plugin_filter() -> *mut AudioProcessor {
    
    todo!();
        /*
            return new TuningworkbenchsynthAudioProcessor();
        */
}
