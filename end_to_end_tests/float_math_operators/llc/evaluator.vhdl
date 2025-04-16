library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity evaluator is
    port (
        clk, input_clk, rst : in std_logic;
        input_time : in unsigned(63 downto 0);
		a : in sfixed(8 downto -23);
		a_en : in std_logic;
		b : in sfixed(8 downto -23);
		b_en : in std_logic;
		c : in sfixed(8 downto -23);
		c_en : in std_logic;
		plus_op_en : in std_logic;
		minus_op_en : in std_logic;
		mult_op_en : in std_logic;
		func_abs_en : in std_logic;
		func_sqrt_en : in std_logic;
		counter_en : in std_logic;
		plus_op : out sfixed(8 downto -23);
		minus_op : out sfixed(8 downto -23);
		mult_op : out sfixed(8 downto -23);
		func_abs : out sfixed(8 downto -23);
		func_sqrt : out sfixed(8 downto -23);
		counter : out signed(31 downto 0);
        done : out std_logic;
        valid : out std_logic
    );
end evaluator;

--* Specification:
--* input a : Float32
--* input b : Float32
--* input c : Float32
--* output plus_op := a + b
--* output minus_op := a - b
--* output mult_op := a * b
--* output func_abs := abs(b)
--* output func_sqrt := sqrt(c)
--* output counter := (counter.offset(by: neg1).defaults(to: 0) + 1)


architecture mixed of evaluator is

    -- Component Declaration
	--* input a : Float32
    component a_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in sfixed(8 downto -23);
		    data_out : out sfixed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* input b : Float32
    component b_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in sfixed(8 downto -23);
		    data_out : out sfixed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* input c : Float32
    component c_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in sfixed(8 downto -23);
		    data_out : out sfixed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* output plus_op := a + b
    component plus_op_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in sfixed(8 downto -23);
			a_data_valid_0 : in std_logic;
			b_0 : in sfixed(8 downto -23);
			b_data_valid_0 : in std_logic;
		    data_out : out sfixed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output minus_op := a - b
    component minus_op_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in sfixed(8 downto -23);
			a_data_valid_0 : in std_logic;
			b_0 : in sfixed(8 downto -23);
			b_data_valid_0 : in std_logic;
		    data_out : out sfixed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output mult_op := a * b
    component mult_op_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in sfixed(8 downto -23);
			a_data_valid_0 : in std_logic;
			b_0 : in sfixed(8 downto -23);
			b_data_valid_0 : in std_logic;
		    data_out : out sfixed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output func_abs := abs(b)
    component func_abs_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			b_0 : in sfixed(8 downto -23);
			b_data_valid_0 : in std_logic;
		    data_out : out sfixed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output func_sqrt := sqrt(c)
    component func_sqrt_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			c_0 : in sfixed(8 downto -23);
			c_data_valid_0 : in std_logic;
		    data_out : out sfixed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output counter := (counter.offset(by: neg1).defaults(to: 0) + 1)
    component counter_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			counter_neg1 : in signed(31 downto 0);
			counter_data_valid_neg1 : in std_logic;
		    data_out : out signed32_array(1 downto 0);
		    data_valid_out : out bit_array(1 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;


    -- Internal Signal Declarations
	signal a_upd : std_logic;
	signal a_upd_done : std_logic;
	signal a_entity_data_0 : sfixed(8 downto -23);
	signal a_entity_data_valid_0 : std_logic;
	signal b_upd : std_logic;
	signal b_upd_done : std_logic;
	signal b_entity_data_0 : sfixed(8 downto -23);
	signal b_entity_data_valid_0 : std_logic;
	signal c_upd : std_logic;
	signal c_upd_done : std_logic;
	signal c_entity_data_0 : sfixed(8 downto -23);
	signal c_entity_data_valid_0 : std_logic;
	signal plus_op_pe : std_logic;
	signal plus_op_eval : std_logic;
	signal plus_op_pe_done : std_logic;
	signal plus_op_eval_done : std_logic;
	signal plus_op_entity_data_0 : sfixed(8 downto -23);
	signal plus_op_entity_data_valid_0 : std_logic;
	signal minus_op_pe : std_logic;
	signal minus_op_eval : std_logic;
	signal minus_op_pe_done : std_logic;
	signal minus_op_eval_done : std_logic;
	signal minus_op_entity_data_0 : sfixed(8 downto -23);
	signal minus_op_entity_data_valid_0 : std_logic;
	signal mult_op_pe : std_logic;
	signal mult_op_eval : std_logic;
	signal mult_op_pe_done : std_logic;
	signal mult_op_eval_done : std_logic;
	signal mult_op_entity_data_0 : sfixed(8 downto -23);
	signal mult_op_entity_data_valid_0 : std_logic;
	signal func_abs_pe : std_logic;
	signal func_abs_eval : std_logic;
	signal func_abs_pe_done : std_logic;
	signal func_abs_eval_done : std_logic;
	signal func_abs_entity_data_0 : sfixed(8 downto -23);
	signal func_abs_entity_data_valid_0 : std_logic;
	signal func_sqrt_pe : std_logic;
	signal func_sqrt_eval : std_logic;
	signal func_sqrt_pe_done : std_logic;
	signal func_sqrt_eval_done : std_logic;
	signal func_sqrt_entity_data_0 : sfixed(8 downto -23);
	signal func_sqrt_entity_data_valid_0 : std_logic;
	signal counter_pe : std_logic;
	signal counter_eval : std_logic;
	signal counter_pe_done : std_logic;
	signal counter_eval_done : std_logic;
	signal counter_entity_data_0 : signed(31 downto 0);
	signal counter_entity_data_valid_0 : std_logic;
	signal counter_entity_data_1 : signed(31 downto 0);
	signal counter_entity_data_valid_1 : std_logic;

    signal upd_and_pe_done : std_logic;
    signal evaluator_done : std_logic;
    signal valid_reg : std_logic;
    signal rst_en_done : std_logic;

begin
    -- Component Instantiation
	--* input a : Float32
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

	--* input b : Float32
    b_entity_instance: b_input_stream_entity
        port map (
            clk => clk,
            rst => rst,
            upd => b_upd,
            data_in => b,
			data_out(0) => b_entity_data_0,
			data_valid_out(0) => b_entity_data_valid_0,
            done_out => b_upd_done
         );

	--* input c : Float32
    c_entity_instance: c_input_stream_entity
        port map (
            clk => clk,
            rst => rst,
            upd => c_upd,
            data_in => c,
			data_out(0) => c_entity_data_0,
			data_valid_out(0) => c_entity_data_valid_0,
            done_out => c_upd_done
         );

	--* output plus_op := a + b
    plus_op_entity_instance: plus_op_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => plus_op_pe,
            eval => plus_op_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			data_out(0) => plus_op_entity_data_0,
			data_valid_out(0) => plus_op_entity_data_valid_0,
            pe_done_out => plus_op_pe_done,
            eval_done_out => plus_op_eval_done
        );

	--* output minus_op := a - b
    minus_op_entity_instance: minus_op_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => minus_op_pe,
            eval => minus_op_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			data_out(0) => minus_op_entity_data_0,
			data_valid_out(0) => minus_op_entity_data_valid_0,
            pe_done_out => minus_op_pe_done,
            eval_done_out => minus_op_eval_done
        );

	--* output mult_op := a * b
    mult_op_entity_instance: mult_op_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => mult_op_pe,
            eval => mult_op_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			data_out(0) => mult_op_entity_data_0,
			data_valid_out(0) => mult_op_entity_data_valid_0,
            pe_done_out => mult_op_pe_done,
            eval_done_out => mult_op_eval_done
        );

	--* output func_abs := abs(b)
    func_abs_entity_instance: func_abs_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => func_abs_pe,
            eval => func_abs_eval,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			data_out(0) => func_abs_entity_data_0,
			data_valid_out(0) => func_abs_entity_data_valid_0,
            pe_done_out => func_abs_pe_done,
            eval_done_out => func_abs_eval_done
        );

	--* output func_sqrt := sqrt(c)
    func_sqrt_entity_instance: func_sqrt_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => func_sqrt_pe,
            eval => func_sqrt_eval,
			c_0 => c_entity_data_0,
			c_data_valid_0 => c_entity_data_valid_0,
			data_out(0) => func_sqrt_entity_data_0,
			data_valid_out(0) => func_sqrt_entity_data_valid_0,
            pe_done_out => func_sqrt_pe_done,
            eval_done_out => func_sqrt_eval_done
        );

	--* output counter := (counter.offset(by: neg1).defaults(to: 0) + 1)
    counter_entity_instance: counter_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => counter_pe,
            eval => counter_eval,
			counter_neg1 => counter_entity_data_1,
			counter_data_valid_neg1 => counter_entity_data_valid_1,
			data_out(0) => counter_entity_data_0,
			data_out(1) => counter_entity_data_1,
			data_valid_out(0) => counter_entity_data_valid_0,
			data_valid_out(1) => counter_entity_data_valid_1,
            pe_done_out => counter_pe_done,
            eval_done_out => counter_eval_done
        );


    process(clk, rst) begin
        if rst = '1' then
            -- Reset Phase
            valid_reg <= '0';
				a_upd <= '0';
				b_upd <= '0';
				c_upd <= '0';
				plus_op_pe <= '0';
				plus_op_eval <= '0';
				minus_op_pe <= '0';
				minus_op_eval <= '0';
				mult_op_pe <= '0';
				mult_op_eval <= '0';
				func_abs_pe <= '0';
				func_abs_eval <= '0';
				func_sqrt_pe <= '0';
				func_sqrt_eval <= '0';
				counter_pe <= '0';
				counter_eval <= '0';
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
					--* - b 
					--* - c 
					a_upd <= a_en;
					b_upd <= b_en;
					c_upd <= c_en;
                    -- Pseudo Evaluation Phase
                    --* Output Streams in Specification 
					--* - plus_op
					--* - minus_op
					--* - mult_op
					--* - func_abs
					--* - func_sqrt
					--* - counter
					plus_op_pe <= plus_op_en;
					minus_op_pe <= minus_op_en;
					mult_op_pe <= mult_op_en;
					func_abs_pe <= func_abs_en;
					func_sqrt_pe <= func_sqrt_en;
					counter_pe <= counter_en;
                    -- Evict Phase
                    --* Sliding Windows in Specification 
                    upd_and_pe_done <= '1';
                    evaluator_done <= '0';
                else
                    -- Eval Phase
					--* output plus_op := a + b
					--* Evaluation Phase of Output Stream plus_op is Influenced by No Lookup
					plus_op_eval <= plus_op_en and upd_and_pe_done;
					--* output minus_op := a - b
					--* Evaluation Phase of Output Stream minus_op is Influenced by No Lookup
					minus_op_eval <= minus_op_en and upd_and_pe_done;
					--* output mult_op := a * b
					--* Evaluation Phase of Output Stream mult_op is Influenced by No Lookup
					mult_op_eval <= mult_op_en and upd_and_pe_done;
					--* output func_abs := abs(b)
					--* Evaluation Phase of Output Stream func_abs is Influenced by No Lookup
					func_abs_eval <= func_abs_en and upd_and_pe_done;
					--* output func_sqrt := sqrt(c)
					--* Evaluation Phase of Output Stream func_sqrt is Influenced by No Lookup
					func_sqrt_eval <= func_sqrt_en and upd_and_pe_done;
					--* output counter := (counter.offset(by: neg1).defaults(to: 0) + 1)
					--* Evaluation Phase of Output Stream counter is Influenced by No Lookup
					counter_eval <= counter_en and upd_and_pe_done;
                    -- SW Update Phase
                    -- SW Request Phase
                    -- Valid Assignment
					valid_reg <= '1' and plus_op_entity_data_valid_0 and minus_op_entity_data_valid_0 and mult_op_entity_data_valid_0 and func_abs_entity_data_valid_0 and func_sqrt_entity_data_valid_0 and counter_entity_data_valid_0;
                    -- Evaluator Done assignment
					upd_and_pe_done <= '1' and (not a_en or a_upd_done) and (not b_en or b_upd_done) and (not c_en or c_upd_done) and (not plus_op_en or plus_op_pe_done) and (not minus_op_en or minus_op_pe_done) and (not mult_op_en or mult_op_pe_done) and (not func_abs_en or func_abs_pe_done) and (not func_sqrt_en or func_sqrt_pe_done) and (not counter_en or counter_pe_done);
					evaluator_done <= upd_and_pe_done and (not plus_op_en or plus_op_eval_done) and (not minus_op_en or minus_op_eval_done) and (not mult_op_en or mult_op_eval_done) and (not func_abs_en or func_abs_eval_done) and (not func_sqrt_en or func_sqrt_eval_done) and (not counter_en or counter_eval_done);
                end if;
            else
                upd_and_pe_done <= '0';
				a_upd <= '0';
				b_upd <= '0';
				c_upd <= '0';
				plus_op_pe <= '0';
				plus_op_eval <= '0';
				minus_op_pe <= '0';
				minus_op_eval <= '0';
				mult_op_pe <= '0';
				mult_op_eval <= '0';
				func_abs_pe <= '0';
				func_abs_eval <= '0';
				func_sqrt_pe <= '0';
				func_sqrt_eval <= '0';
				counter_pe <= '0';
				counter_eval <= '0';
            end if;
        end if;
    end process;

    -- output port assignment
	plus_op <= plus_op_entity_data_0;
	minus_op <= minus_op_entity_data_0;
	mult_op <= mult_op_entity_data_0;
	func_abs <= func_abs_entity_data_0;
	func_sqrt <= func_sqrt_entity_data_0;
	counter <= counter_entity_data_0;
    valid <= valid_reg;
    done <= evaluator_done;

end mixed;