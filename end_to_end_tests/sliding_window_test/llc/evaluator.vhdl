library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity evaluator is
    port (
        clk, input_clk, rst : in std_logic;
        input_time : in unsigned(63 downto 0);
		a : in signed(31 downto 0);
		a_en : in std_logic;
		s_s_en : in std_logic;
		c_s_en : in std_logic;
		av_s_en : in std_logic;
		a_u_en : in std_logic;
		s_u_en : in std_logic;
		c_u_en : in std_logic;
		av_u_en : in std_logic;
		s_s : out signed(31 downto 0);
		c_s : out unsigned(63 downto 0);
		av_s : out signed(31 downto 0);
		a_u : out unsigned(31 downto 0);
		s_u : out unsigned(31 downto 0);
		c_u : out unsigned(63 downto 0);
		av_u : out unsigned(31 downto 0);
        done : out std_logic;
        valid : out std_logic
    );
end evaluator;

--* Specification:
--* input a : Int32
--* output s_s := a.aggregate(over: 0.3s, using: sum)
--* output c_s := a.aggregate(over: 0.3s, using: count)
--* output av_s := a.aggregate(over: 0.3s, using: avg).defaults(to: 10)
--* output a_u := cast(a)
--* output s_u := a_u.aggregate(over: 0.3s, using: sum)
--* output c_u := a_u.aggregate(over: 0.3s, using: count)
--* output av_u := a_u.aggregate(over: 0.3s, using: avg).defaults(to: 10)


architecture mixed of evaluator is

    -- Component Declaration
	--* input a : Int32
    component a_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in signed(31 downto 0);
		    data_out : out signed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* output s_s := a.aggregate(over: 0.3s, using: sum)
    component s_s_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_sum_0_sw : in signed(31 downto 0);
			a_sum_0_sw_data_valid : in std_logic;
		    data_out : out signed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output c_s := a.aggregate(over: 0.3s, using: count)
    component c_s_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_count_1_sw : in unsigned(63 downto 0);
			a_count_1_sw_data_valid : in std_logic;
		    data_out : out unsigned64_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output av_s := a.aggregate(over: 0.3s, using: avg).defaults(to: 10)
    component av_s_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_avg_2_sw : in signed(31 downto 0);
			a_avg_2_sw_data_valid : in std_logic;
		    data_out : out signed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output a_u := cast(a)
    component a_u_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in signed(31 downto 0);
			a_data_valid_0 : in std_logic;
		    data_out : out unsigned32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output s_u := a_u.aggregate(over: 0.3s, using: sum)
    component s_u_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_u_sum_3_sw : in unsigned(31 downto 0);
			a_u_sum_3_sw_data_valid : in std_logic;
		    data_out : out unsigned32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output c_u := a_u.aggregate(over: 0.3s, using: count)
    component c_u_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_u_count_4_sw : in unsigned(63 downto 0);
			a_u_count_4_sw_data_valid : in std_logic;
		    data_out : out unsigned64_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output av_u := a_u.aggregate(over: 0.3s, using: avg).defaults(to: 10)
    component av_u_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_u_avg_5_sw : in unsigned(31 downto 0);
			a_u_avg_5_sw_data_valid : in std_logic;
		    data_out : out unsigned32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* a.aggregate(over: 0.3 s, using: sum)
    component a_sum_0_sliding_window_entity
	    port (
		    clk, rst : in std_logic;
		    evict, upd, request : in std_logic;
            time_in : in unsigned(63 downto 0);
            data_in : in signed(31 downto 0);
            data_out : out signed(31 downto 0);
            data_valid_out : out std_logic;
            evict_done_out : out std_logic;
            upd_done_out : out std_logic;
            request_done_out : out std_logic
	    );
    end component;

	--* a.aggregate(over: 0.3 s, using: count)
    component a_count_1_sliding_window_entity
	    port (
		    clk, rst : in std_logic;
		    evict, upd, request : in std_logic;
            time_in : in unsigned(63 downto 0);
            data_in : in signed(31 downto 0);
            data_out : out unsigned(63 downto 0);
            data_valid_out : out std_logic;
            evict_done_out : out std_logic;
            upd_done_out : out std_logic;
            request_done_out : out std_logic
	    );
    end component;

	--* a.aggregate(over: 0.3 s, using: avg)
    component a_avg_2_sliding_window_entity
	    port (
		    clk, rst : in std_logic;
		    evict, upd, request : in std_logic;
            time_in : in unsigned(63 downto 0);
            data_in : in signed(31 downto 0);
            data_out : out signed(31 downto 0);
            data_valid_out : out std_logic;
            evict_done_out : out std_logic;
            upd_done_out : out std_logic;
            request_done_out : out std_logic
	    );
    end component;

	--* a_u.aggregate(over: 0.3 s, using: sum)
    component a_u_sum_3_sliding_window_entity
	    port (
		    clk, rst : in std_logic;
		    evict, upd, request : in std_logic;
            time_in : in unsigned(63 downto 0);
            data_in : in unsigned(31 downto 0);
            data_out : out unsigned(31 downto 0);
            data_valid_out : out std_logic;
            evict_done_out : out std_logic;
            upd_done_out : out std_logic;
            request_done_out : out std_logic
	    );
    end component;

	--* a_u.aggregate(over: 0.3 s, using: count)
    component a_u_count_4_sliding_window_entity
	    port (
		    clk, rst : in std_logic;
		    evict, upd, request : in std_logic;
            time_in : in unsigned(63 downto 0);
            data_in : in unsigned(31 downto 0);
            data_out : out unsigned(63 downto 0);
            data_valid_out : out std_logic;
            evict_done_out : out std_logic;
            upd_done_out : out std_logic;
            request_done_out : out std_logic
	    );
    end component;

	--* a_u.aggregate(over: 0.3 s, using: avg)
    component a_u_avg_5_sliding_window_entity
	    port (
		    clk, rst : in std_logic;
		    evict, upd, request : in std_logic;
            time_in : in unsigned(63 downto 0);
            data_in : in unsigned(31 downto 0);
            data_out : out unsigned(31 downto 0);
            data_valid_out : out std_logic;
            evict_done_out : out std_logic;
            upd_done_out : out std_logic;
            request_done_out : out std_logic
	    );
    end component;


    -- Internal Signal Declarations
	signal a_upd : std_logic;
	signal a_upd_done : std_logic;
	signal a_entity_data_0 : signed(31 downto 0);
	signal a_entity_data_valid_0 : std_logic;
	signal s_s_pe : std_logic;
	signal s_s_eval : std_logic;
	signal s_s_pe_done : std_logic;
	signal s_s_eval_done : std_logic;
	signal s_s_entity_data_0 : signed(31 downto 0);
	signal s_s_entity_data_valid_0 : std_logic;
	signal c_s_pe : std_logic;
	signal c_s_eval : std_logic;
	signal c_s_pe_done : std_logic;
	signal c_s_eval_done : std_logic;
	signal c_s_entity_data_0 : unsigned(63 downto 0);
	signal c_s_entity_data_valid_0 : std_logic;
	signal av_s_pe : std_logic;
	signal av_s_eval : std_logic;
	signal av_s_pe_done : std_logic;
	signal av_s_eval_done : std_logic;
	signal av_s_entity_data_0 : signed(31 downto 0);
	signal av_s_entity_data_valid_0 : std_logic;
	signal a_u_pe : std_logic;
	signal a_u_eval : std_logic;
	signal a_u_pe_done : std_logic;
	signal a_u_eval_done : std_logic;
	signal a_u_entity_data_0 : unsigned(31 downto 0);
	signal a_u_entity_data_valid_0 : std_logic;
	signal s_u_pe : std_logic;
	signal s_u_eval : std_logic;
	signal s_u_pe_done : std_logic;
	signal s_u_eval_done : std_logic;
	signal s_u_entity_data_0 : unsigned(31 downto 0);
	signal s_u_entity_data_valid_0 : std_logic;
	signal c_u_pe : std_logic;
	signal c_u_eval : std_logic;
	signal c_u_pe_done : std_logic;
	signal c_u_eval_done : std_logic;
	signal c_u_entity_data_0 : unsigned(63 downto 0);
	signal c_u_entity_data_valid_0 : std_logic;
	signal av_u_pe : std_logic;
	signal av_u_eval : std_logic;
	signal av_u_pe_done : std_logic;
	signal av_u_eval_done : std_logic;
	signal av_u_entity_data_0 : unsigned(31 downto 0);
	signal av_u_entity_data_valid_0 : std_logic;
	signal a_sum_0_evict : std_logic;
	signal a_sum_0_upd : std_logic;
	signal a_sum_0_request : std_logic;
	signal a_sum_0_entity_data : signed(31 downto 0);
	signal a_sum_0_entity_data_valid : std_logic;
	signal a_sum_0_evict_done : std_logic;
	signal a_sum_0_upd_done : std_logic;
	signal a_sum_0_request_done : std_logic;
	signal a_count_1_evict : std_logic;
	signal a_count_1_upd : std_logic;
	signal a_count_1_request : std_logic;
	signal a_count_1_entity_data : unsigned(63 downto 0);
	signal a_count_1_entity_data_valid : std_logic;
	signal a_count_1_evict_done : std_logic;
	signal a_count_1_upd_done : std_logic;
	signal a_count_1_request_done : std_logic;
	signal a_avg_2_evict : std_logic;
	signal a_avg_2_upd : std_logic;
	signal a_avg_2_request : std_logic;
	signal a_avg_2_entity_data : signed(31 downto 0);
	signal a_avg_2_entity_data_valid : std_logic;
	signal a_avg_2_evict_done : std_logic;
	signal a_avg_2_upd_done : std_logic;
	signal a_avg_2_request_done : std_logic;
	signal a_u_sum_3_evict : std_logic;
	signal a_u_sum_3_upd : std_logic;
	signal a_u_sum_3_request : std_logic;
	signal a_u_sum_3_entity_data : unsigned(31 downto 0);
	signal a_u_sum_3_entity_data_valid : std_logic;
	signal a_u_sum_3_evict_done : std_logic;
	signal a_u_sum_3_upd_done : std_logic;
	signal a_u_sum_3_request_done : std_logic;
	signal a_u_count_4_evict : std_logic;
	signal a_u_count_4_upd : std_logic;
	signal a_u_count_4_request : std_logic;
	signal a_u_count_4_entity_data : unsigned(63 downto 0);
	signal a_u_count_4_entity_data_valid : std_logic;
	signal a_u_count_4_evict_done : std_logic;
	signal a_u_count_4_upd_done : std_logic;
	signal a_u_count_4_request_done : std_logic;
	signal a_u_avg_5_evict : std_logic;
	signal a_u_avg_5_upd : std_logic;
	signal a_u_avg_5_request : std_logic;
	signal a_u_avg_5_entity_data : unsigned(31 downto 0);
	signal a_u_avg_5_entity_data_valid : std_logic;
	signal a_u_avg_5_evict_done : std_logic;
	signal a_u_avg_5_upd_done : std_logic;
	signal a_u_avg_5_request_done : std_logic;

    signal upd_and_pe_done : std_logic;
    signal evaluator_done : std_logic;
    signal valid_reg : std_logic;
    signal rst_en_done : std_logic;

begin
    -- Component Instantiation
	--* input a : Int32
    a_entity_instance: a_input_stream_entity
        port map (
            clk => clk,
            rst => rst,
            upd => a_upd,
            data_in => a,
			data_out(0) => a_entity_data_0,
			data_valid_out(0) => a_entity_data_valid_0,
            done_out => a_upd_done
         );

	--* output s_s := a.aggregate(over: 0.3s, using: sum)
    s_s_entity_instance: s_s_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => s_s_pe,
            eval => s_s_eval,
			a_sum_0_sw => a_sum_0_entity_data,
			a_sum_0_sw_data_valid => a_sum_0_entity_data_valid,
			data_out(0) => s_s_entity_data_0,
			data_valid_out(0) => s_s_entity_data_valid_0,
            pe_done_out => s_s_pe_done,
            eval_done_out => s_s_eval_done
        );

	--* output c_s := a.aggregate(over: 0.3s, using: count)
    c_s_entity_instance: c_s_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => c_s_pe,
            eval => c_s_eval,
			a_count_1_sw => a_count_1_entity_data,
			a_count_1_sw_data_valid => a_count_1_entity_data_valid,
			data_out(0) => c_s_entity_data_0,
			data_valid_out(0) => c_s_entity_data_valid_0,
            pe_done_out => c_s_pe_done,
            eval_done_out => c_s_eval_done
        );

	--* output av_s := a.aggregate(over: 0.3s, using: avg).defaults(to: 10)
    av_s_entity_instance: av_s_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => av_s_pe,
            eval => av_s_eval,
			a_avg_2_sw => a_avg_2_entity_data,
			a_avg_2_sw_data_valid => a_avg_2_entity_data_valid,
			data_out(0) => av_s_entity_data_0,
			data_valid_out(0) => av_s_entity_data_valid_0,
            pe_done_out => av_s_pe_done,
            eval_done_out => av_s_eval_done
        );

	--* output a_u := cast(a)
    a_u_entity_instance: a_u_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => a_u_pe,
            eval => a_u_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			data_out(0) => a_u_entity_data_0,
			data_valid_out(0) => a_u_entity_data_valid_0,
            pe_done_out => a_u_pe_done,
            eval_done_out => a_u_eval_done
        );

	--* output s_u := a_u.aggregate(over: 0.3s, using: sum)
    s_u_entity_instance: s_u_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => s_u_pe,
            eval => s_u_eval,
			a_u_sum_3_sw => a_u_sum_3_entity_data,
			a_u_sum_3_sw_data_valid => a_u_sum_3_entity_data_valid,
			data_out(0) => s_u_entity_data_0,
			data_valid_out(0) => s_u_entity_data_valid_0,
            pe_done_out => s_u_pe_done,
            eval_done_out => s_u_eval_done
        );

	--* output c_u := a_u.aggregate(over: 0.3s, using: count)
    c_u_entity_instance: c_u_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => c_u_pe,
            eval => c_u_eval,
			a_u_count_4_sw => a_u_count_4_entity_data,
			a_u_count_4_sw_data_valid => a_u_count_4_entity_data_valid,
			data_out(0) => c_u_entity_data_0,
			data_valid_out(0) => c_u_entity_data_valid_0,
            pe_done_out => c_u_pe_done,
            eval_done_out => c_u_eval_done
        );

	--* output av_u := a_u.aggregate(over: 0.3s, using: avg).defaults(to: 10)
    av_u_entity_instance: av_u_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => av_u_pe,
            eval => av_u_eval,
			a_u_avg_5_sw => a_u_avg_5_entity_data,
			a_u_avg_5_sw_data_valid => a_u_avg_5_entity_data_valid,
			data_out(0) => av_u_entity_data_0,
			data_valid_out(0) => av_u_entity_data_valid_0,
            pe_done_out => av_u_pe_done,
            eval_done_out => av_u_eval_done
        );

	--* a.aggregate(over: 0.3 s, using: sum)
    a_sum_0_sliding_window_entity_instance: a_sum_0_sliding_window_entity
        port map (
            clk => clk,
            rst => rst,
            evict => a_sum_0_evict,
            upd => a_sum_0_upd,
            request => a_sum_0_request,
            time_in => input_time,
            data_in => a_entity_data_0,
            data_out => a_sum_0_entity_data,
            data_valid_out => a_sum_0_entity_data_valid,
            evict_done_out => a_sum_0_evict_done,
            upd_done_out => a_sum_0_upd_done,
            request_done_out => a_sum_0_request_done
        );

	--* a.aggregate(over: 0.3 s, using: count)
    a_count_1_sliding_window_entity_instance: a_count_1_sliding_window_entity
        port map (
            clk => clk,
            rst => rst,
            evict => a_count_1_evict,
            upd => a_count_1_upd,
            request => a_count_1_request,
            time_in => input_time,
            data_in => a_entity_data_0,
            data_out => a_count_1_entity_data,
            data_valid_out => a_count_1_entity_data_valid,
            evict_done_out => a_count_1_evict_done,
            upd_done_out => a_count_1_upd_done,
            request_done_out => a_count_1_request_done
        );

	--* a.aggregate(over: 0.3 s, using: avg)
    a_avg_2_sliding_window_entity_instance: a_avg_2_sliding_window_entity
        port map (
            clk => clk,
            rst => rst,
            evict => a_avg_2_evict,
            upd => a_avg_2_upd,
            request => a_avg_2_request,
            time_in => input_time,
            data_in => a_entity_data_0,
            data_out => a_avg_2_entity_data,
            data_valid_out => a_avg_2_entity_data_valid,
            evict_done_out => a_avg_2_evict_done,
            upd_done_out => a_avg_2_upd_done,
            request_done_out => a_avg_2_request_done
        );

	--* a_u.aggregate(over: 0.3 s, using: sum)
    a_u_sum_3_sliding_window_entity_instance: a_u_sum_3_sliding_window_entity
        port map (
            clk => clk,
            rst => rst,
            evict => a_u_sum_3_evict,
            upd => a_u_sum_3_upd,
            request => a_u_sum_3_request,
            time_in => input_time,
            data_in => a_u_entity_data_0,
            data_out => a_u_sum_3_entity_data,
            data_valid_out => a_u_sum_3_entity_data_valid,
            evict_done_out => a_u_sum_3_evict_done,
            upd_done_out => a_u_sum_3_upd_done,
            request_done_out => a_u_sum_3_request_done
        );

	--* a_u.aggregate(over: 0.3 s, using: count)
    a_u_count_4_sliding_window_entity_instance: a_u_count_4_sliding_window_entity
        port map (
            clk => clk,
            rst => rst,
            evict => a_u_count_4_evict,
            upd => a_u_count_4_upd,
            request => a_u_count_4_request,
            time_in => input_time,
            data_in => a_u_entity_data_0,
            data_out => a_u_count_4_entity_data,
            data_valid_out => a_u_count_4_entity_data_valid,
            evict_done_out => a_u_count_4_evict_done,
            upd_done_out => a_u_count_4_upd_done,
            request_done_out => a_u_count_4_request_done
        );

	--* a_u.aggregate(over: 0.3 s, using: avg)
    a_u_avg_5_sliding_window_entity_instance: a_u_avg_5_sliding_window_entity
        port map (
            clk => clk,
            rst => rst,
            evict => a_u_avg_5_evict,
            upd => a_u_avg_5_upd,
            request => a_u_avg_5_request,
            time_in => input_time,
            data_in => a_u_entity_data_0,
            data_out => a_u_avg_5_entity_data,
            data_valid_out => a_u_avg_5_entity_data_valid,
            evict_done_out => a_u_avg_5_evict_done,
            upd_done_out => a_u_avg_5_upd_done,
            request_done_out => a_u_avg_5_request_done
        );


    process(clk, rst) begin
        if rst = '1' then
            -- Reset Phase
            valid_reg <= '0';
				a_upd <= '0';
				s_s_pe <= '0';
				s_s_eval <= '0';
				c_s_pe <= '0';
				c_s_eval <= '0';
				av_s_pe <= '0';
				av_s_eval <= '0';
				a_u_pe <= '0';
				a_u_eval <= '0';
				s_u_pe <= '0';
				s_u_eval <= '0';
				c_u_pe <= '0';
				c_u_eval <= '0';
				av_u_pe <= '0';
				av_u_eval <= '0';
				a_sum_0_evict <= '0';
				a_sum_0_upd <= '0';
				a_sum_0_request <= '0';
				a_count_1_evict <= '0';
				a_count_1_upd <= '0';
				a_count_1_request <= '0';
				a_avg_2_evict <= '0';
				a_avg_2_upd <= '0';
				a_avg_2_request <= '0';
				a_u_sum_3_evict <= '0';
				a_u_sum_3_upd <= '0';
				a_u_sum_3_request <= '0';
				a_u_count_4_evict <= '0';
				a_u_count_4_upd <= '0';
				a_u_count_4_request <= '0';
				a_u_avg_5_evict <= '0';
				a_u_avg_5_upd <= '0';
				a_u_avg_5_request <= '0';
            upd_and_pe_done <= '1';
            evaluator_done <= '1';
            rst_en_done <= '0';
        elsif rising_edge(clk) then
            -- Logic Phase
            if input_clk = '1' then
                if upd_and_pe_done = '0' then
                    -- Input Stream Updates
                    --* Input Streams in Specification 
					--* - a 
					a_upd <= a_en;
                    -- Pseudo Evaluation Phase
                    --* Output Streams in Specification 
					--* - s_s
					--* - c_s
					--* - av_s
					--* - a_u
					--* - s_u
					--* - c_u
					--* - av_u
					s_s_pe <= s_s_en;
					c_s_pe <= c_s_en;
					av_s_pe <= av_s_en;
					a_u_pe <= a_u_en;
					s_u_pe <= s_u_en;
					c_u_pe <= c_u_en;
					av_u_pe <= av_u_en;
                    -- Evict Phase
                    --* Sliding Windows in Specification 
					--* - a.aggregate(over: 0.3 s, using: sum)
					--* - a.aggregate(over: 0.3 s, using: count)
					--* - a.aggregate(over: 0.3 s, using: avg)
					--* - a_u.aggregate(over: 0.3 s, using: sum)
					--* - a_u.aggregate(over: 0.3 s, using: count)
					--* - a_u.aggregate(over: 0.3 s, using: avg)
					a_sum_0_evict <= '1';
					a_count_1_evict <= '1';
					a_avg_2_evict <= '1';
					a_u_sum_3_evict <= '1';
					a_u_count_4_evict <= '1';
					a_u_avg_5_evict <= '1';
                    upd_and_pe_done <= '1';
                    evaluator_done <= '0';
                else
                    -- Eval Phase
					--* output s_s := a.aggregate(over: 0.3s, using: sum)
					--* Evaluation Phase of Output Stream s_s is Influenced by the following Lookups: 
					--* - Window Lookup: a.aggregate(over: 0.3 s, using: sum)
					s_s_eval <= s_s_en and upd_and_pe_done and a_sum_0_request_done;
					--* output c_s := a.aggregate(over: 0.3s, using: count)
					--* Evaluation Phase of Output Stream c_s is Influenced by the following Lookups: 
					--* - Window Lookup: a.aggregate(over: 0.3 s, using: count)
					c_s_eval <= c_s_en and upd_and_pe_done and a_count_1_request_done;
					--* output av_s := a.aggregate(over: 0.3s, using: avg).defaults(to: 10)
					--* Evaluation Phase of Output Stream av_s is Influenced by the following Lookups: 
					--* - Window Lookup: a.aggregate(over: 0.3 s, using: avg)
					av_s_eval <= av_s_en and upd_and_pe_done and a_avg_2_request_done;
					--* output a_u := cast(a)
					--* Evaluation Phase of Output Stream a_u is Influenced by No Lookup
					a_u_eval <= a_u_en and upd_and_pe_done;
					--* output s_u := a_u.aggregate(over: 0.3s, using: sum)
					--* Evaluation Phase of Output Stream s_u is Influenced by the following Lookups: 
					--* - Window Lookup: a_u.aggregate(over: 0.3 s, using: sum)
					s_u_eval <= s_u_en and upd_and_pe_done and a_u_sum_3_request_done;
					--* output c_u := a_u.aggregate(over: 0.3s, using: count)
					--* Evaluation Phase of Output Stream c_u is Influenced by the following Lookups: 
					--* - Window Lookup: a_u.aggregate(over: 0.3 s, using: count)
					c_u_eval <= c_u_en and upd_and_pe_done and a_u_count_4_request_done;
					--* output av_u := a_u.aggregate(over: 0.3s, using: avg).defaults(to: 10)
					--* Evaluation Phase of Output Stream av_u is Influenced by the following Lookups: 
					--* - Window Lookup: a_u.aggregate(over: 0.3 s, using: avg)
					av_u_eval <= av_u_en and upd_and_pe_done and a_u_avg_5_request_done;
                    -- SW Update Phase
					--* - a.aggregate(over: 0.3 s, using: sum) aggregates over a
					a_sum_0_upd <= a_upd_done and upd_and_pe_done;
					--* - a.aggregate(over: 0.3 s, using: count) aggregates over a
					a_count_1_upd <= a_upd_done and upd_and_pe_done;
					--* - a.aggregate(over: 0.3 s, using: avg) aggregates over a
					a_avg_2_upd <= a_upd_done and upd_and_pe_done;
					--* - a_u.aggregate(over: 0.3 s, using: sum) aggregates over a_u
					a_u_sum_3_upd <= a_u_eval_done and upd_and_pe_done;
					--* - a_u.aggregate(over: 0.3 s, using: count) aggregates over a_u
					a_u_count_4_upd <= a_u_eval_done and upd_and_pe_done;
					--* - a_u.aggregate(over: 0.3 s, using: avg) aggregates over a_u
					a_u_avg_5_upd <= a_u_eval_done and upd_and_pe_done;
                    -- SW Request Phase
					--* a.aggregate(over: 0.3 s, using: sum) has Source s_s
					a_sum_0_request <= s_s_en and upd_and_pe_done and (not a_en or a_sum_0_upd_done);
					--* a.aggregate(over: 0.3 s, using: count) has Source c_s
					a_count_1_request <= c_s_en and upd_and_pe_done and (not a_en or a_count_1_upd_done);
					--* a.aggregate(over: 0.3 s, using: avg) has Source av_s
					a_avg_2_request <= av_s_en and upd_and_pe_done and (not a_en or a_avg_2_upd_done);
					--* a_u.aggregate(over: 0.3 s, using: sum) has Source s_u
					a_u_sum_3_request <= s_u_en and upd_and_pe_done and (not a_u_en or a_u_sum_3_upd_done);
					--* a_u.aggregate(over: 0.3 s, using: count) has Source c_u
					a_u_count_4_request <= c_u_en and upd_and_pe_done and (not a_u_en or a_u_count_4_upd_done);
					--* a_u.aggregate(over: 0.3 s, using: avg) has Source av_u
					a_u_avg_5_request <= av_u_en and upd_and_pe_done and (not a_u_en or a_u_avg_5_upd_done);
                    -- Valid Assignment
					valid_reg <= '1' and s_s_entity_data_valid_0 and c_s_entity_data_valid_0 and av_s_entity_data_valid_0 and a_u_entity_data_valid_0 and s_u_entity_data_valid_0 and c_u_entity_data_valid_0 and av_u_entity_data_valid_0;
                    -- Evaluator Done assignment
					upd_and_pe_done <= '1' and (not a_en or a_upd_done) and (not s_s_en or s_s_pe_done) and (not c_s_en or c_s_pe_done) and (not av_s_en or av_s_pe_done) and (not a_u_en or a_u_pe_done) and (not s_u_en or s_u_pe_done) and (not c_u_en or c_u_pe_done) and (not av_u_en or av_u_pe_done) and (not a_en or a_sum_0_evict_done) and (not a_en or a_count_1_evict_done) and (not a_en or a_avg_2_evict_done) and (not a_u_en or a_u_sum_3_evict_done) and (not a_u_en or a_u_count_4_evict_done) and (not a_u_en or a_u_avg_5_evict_done);
					evaluator_done <= upd_and_pe_done and (not s_s_en or a_sum_0_request_done) and (not s_s_en or s_s_eval_done) and (not c_s_en or a_count_1_request_done) and (not c_s_en or c_s_eval_done) and (not av_s_en or a_avg_2_request_done) and (not av_s_en or av_s_eval_done) and (not a_u_en or a_u_eval_done) and (not s_u_en or a_u_sum_3_request_done) and (not s_u_en or s_u_eval_done) and (not c_u_en or a_u_count_4_request_done) and (not c_u_en or c_u_eval_done) and (not av_u_en or a_u_avg_5_request_done) and (not av_u_en or av_u_eval_done) and (not a_en or a_sum_0_upd_done) and (not a_en or a_count_1_upd_done) and (not a_en or a_avg_2_upd_done) and (not a_u_en or a_u_sum_3_upd_done) and (not a_u_en or a_u_count_4_upd_done) and (not a_u_en or a_u_avg_5_upd_done);
                end if;
            else
                upd_and_pe_done <= '0';
				a_upd <= '0';
				s_s_pe <= '0';
				s_s_eval <= '0';
				c_s_pe <= '0';
				c_s_eval <= '0';
				av_s_pe <= '0';
				av_s_eval <= '0';
				a_u_pe <= '0';
				a_u_eval <= '0';
				s_u_pe <= '0';
				s_u_eval <= '0';
				c_u_pe <= '0';
				c_u_eval <= '0';
				av_u_pe <= '0';
				av_u_eval <= '0';
				a_sum_0_evict <= '0';
				a_sum_0_upd <= '0';
				a_sum_0_request <= '0';
				a_count_1_evict <= '0';
				a_count_1_upd <= '0';
				a_count_1_request <= '0';
				a_avg_2_evict <= '0';
				a_avg_2_upd <= '0';
				a_avg_2_request <= '0';
				a_u_sum_3_evict <= '0';
				a_u_sum_3_upd <= '0';
				a_u_sum_3_request <= '0';
				a_u_count_4_evict <= '0';
				a_u_count_4_upd <= '0';
				a_u_count_4_request <= '0';
				a_u_avg_5_evict <= '0';
				a_u_avg_5_upd <= '0';
				a_u_avg_5_request <= '0';
            end if;
        end if;
    end process;

    -- output port assignment
	s_s <= s_s_entity_data_0;
	c_s <= c_s_entity_data_0;
	av_s <= av_s_entity_data_0;
	a_u <= a_u_entity_data_0;
	s_u <= s_u_entity_data_0;
	c_u <= c_u_entity_data_0;
	av_u <= av_u_entity_data_0;
    valid <= valid_reg;
    done <= evaluator_done;

end mixed;