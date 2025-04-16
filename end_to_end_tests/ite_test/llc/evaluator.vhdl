library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity evaluator is
    port (
        clk, input_clk, rst : in std_logic;
        input_time : in unsigned(63 downto 0);
		a : in signed(7 downto 0);
		a_en : in std_logic;
		b : in signed(15 downto 0);
		b_en : in std_logic;
		val : in std_logic;
		val_en : in std_logic;
		c_en : in std_logic;
		d_en : in std_logic;
		e_en : in std_logic;
		counter_en : in std_logic;
		c : out unsigned(7 downto 0);
		d : out signed(15 downto 0);
		e : out signed(7 downto 0);
		counter : out signed(63 downto 0);
        done : out std_logic;
        valid : out std_logic
    );
end evaluator;

--* Specification:
--* input a : Int8
--* input b : Int16
--* input val : Bool
--* output c := cast(b)
--* output d := if val then (cast(a) + b) else (cast(a) * b)
--* output e := if (b < 3) then (if val then a else cast(b) + 4) else cast(b)
--* output counter := (counter.offset(by: neg1).defaults(to: 0) + 1)


architecture mixed of evaluator is

    -- Component Declaration
	--* input a : Int8
    component a_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in signed(7 downto 0);
		    data_out : out signed8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* input b : Int16
    component b_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in signed(15 downto 0);
		    data_out : out signed16_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* input val : Bool
    component val_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* output c := cast(b)
    component c_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			b_0 : in signed(15 downto 0);
			b_data_valid_0 : in std_logic;
		    data_out : out unsigned8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output d := if val then (cast(a) + b) else (cast(a) * b)
    component d_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in signed(7 downto 0);
			a_data_valid_0 : in std_logic;
			b_0 : in signed(15 downto 0);
			b_data_valid_0 : in std_logic;
			val_0 : in std_logic;
			val_data_valid_0 : in std_logic;
		    data_out : out signed16_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output e := if (b < 3) then (if val then a else cast(b) + 4) else cast(b)
    component e_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in signed(7 downto 0);
			a_data_valid_0 : in std_logic;
			b_0 : in signed(15 downto 0);
			b_data_valid_0 : in std_logic;
			val_0 : in std_logic;
			val_data_valid_0 : in std_logic;
		    data_out : out signed8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output counter := (counter.offset(by: neg1).defaults(to: 0) + 1)
    component counter_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			counter_neg1 : in signed(63 downto 0);
			counter_data_valid_neg1 : in std_logic;
		    data_out : out signed64_array(1 downto 0);
		    data_valid_out : out bit_array(1 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;


    -- Internal Signal Declarations
	signal a_upd : std_logic;
	signal a_upd_done : std_logic;
	signal a_entity_data_0 : signed(7 downto 0);
	signal a_entity_data_valid_0 : std_logic;
	signal b_upd : std_logic;
	signal b_upd_done : std_logic;
	signal b_entity_data_0 : signed(15 downto 0);
	signal b_entity_data_valid_0 : std_logic;
	signal val_upd : std_logic;
	signal val_upd_done : std_logic;
	signal val_entity_data_0 : std_logic;
	signal val_entity_data_valid_0 : std_logic;
	signal c_pe : std_logic;
	signal c_eval : std_logic;
	signal c_pe_done : std_logic;
	signal c_eval_done : std_logic;
	signal c_entity_data_0 : unsigned(7 downto 0);
	signal c_entity_data_valid_0 : std_logic;
	signal d_pe : std_logic;
	signal d_eval : std_logic;
	signal d_pe_done : std_logic;
	signal d_eval_done : std_logic;
	signal d_entity_data_0 : signed(15 downto 0);
	signal d_entity_data_valid_0 : std_logic;
	signal e_pe : std_logic;
	signal e_eval : std_logic;
	signal e_pe_done : std_logic;
	signal e_eval_done : std_logic;
	signal e_entity_data_0 : signed(7 downto 0);
	signal e_entity_data_valid_0 : std_logic;
	signal counter_pe : std_logic;
	signal counter_eval : std_logic;
	signal counter_pe_done : std_logic;
	signal counter_eval_done : std_logic;
	signal counter_entity_data_0 : signed(63 downto 0);
	signal counter_entity_data_valid_0 : std_logic;
	signal counter_entity_data_1 : signed(63 downto 0);
	signal counter_entity_data_valid_1 : std_logic;

    signal upd_and_pe_done : std_logic;
    signal evaluator_done : std_logic;
    signal valid_reg : std_logic;
    signal rst_en_done : std_logic;

begin
    -- Component Instantiation
	--* input a : Int8
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

	--* input b : Int16
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

	--* input val : Bool
    val_entity_instance: val_input_stream_entity
        port map (
            clk => clk,
            rst => rst,
            upd => val_upd,
            data_in => val,
			data_out(0) => val_entity_data_0,
			data_valid_out(0) => val_entity_data_valid_0,
            done_out => val_upd_done
         );

	--* output c := cast(b)
    c_entity_instance: c_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => c_pe,
            eval => c_eval,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			data_out(0) => c_entity_data_0,
			data_valid_out(0) => c_entity_data_valid_0,
            pe_done_out => c_pe_done,
            eval_done_out => c_eval_done
        );

	--* output d := if val then (cast(a) + b) else (cast(a) * b)
    d_entity_instance: d_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => d_pe,
            eval => d_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			val_0 => val_entity_data_0,
			val_data_valid_0 => val_entity_data_valid_0,
			data_out(0) => d_entity_data_0,
			data_valid_out(0) => d_entity_data_valid_0,
            pe_done_out => d_pe_done,
            eval_done_out => d_eval_done
        );

	--* output e := if (b < 3) then (if val then a else cast(b) + 4) else cast(b)
    e_entity_instance: e_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => e_pe,
            eval => e_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			val_0 => val_entity_data_0,
			val_data_valid_0 => val_entity_data_valid_0,
			data_out(0) => e_entity_data_0,
			data_valid_out(0) => e_entity_data_valid_0,
            pe_done_out => e_pe_done,
            eval_done_out => e_eval_done
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
				val_upd <= '0';
				c_pe <= '0';
				c_eval <= '0';
				d_pe <= '0';
				d_eval <= '0';
				e_pe <= '0';
				e_eval <= '0';
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
					--* - val 
					a_upd <= a_en;
					b_upd <= b_en;
					val_upd <= val_en;
                    -- Pseudo Evaluation Phase
                    --* Output Streams in Specification 
					--* - c
					--* - d
					--* - e
					--* - counter
					c_pe <= c_en;
					d_pe <= d_en;
					e_pe <= e_en;
					counter_pe <= counter_en;
                    -- Evict Phase
                    --* Sliding Windows in Specification 
                    upd_and_pe_done <= '1';
                    evaluator_done <= '0';
                else
                    -- Eval Phase
					--* output c := cast(b)
					--* Evaluation Phase of Output Stream c is Influenced by No Lookup
					c_eval <= c_en and upd_and_pe_done;
					--* output d := if val then (cast(a) + b) else (cast(a) * b)
					--* Evaluation Phase of Output Stream d is Influenced by No Lookup
					d_eval <= d_en and upd_and_pe_done;
					--* output e := if (b < 3) then (if val then a else cast(b) + 4) else cast(b)
					--* Evaluation Phase of Output Stream e is Influenced by No Lookup
					e_eval <= e_en and upd_and_pe_done;
					--* output counter := (counter.offset(by: neg1).defaults(to: 0) + 1)
					--* Evaluation Phase of Output Stream counter is Influenced by No Lookup
					counter_eval <= counter_en and upd_and_pe_done;
                    -- SW Update Phase
                    -- SW Request Phase
                    -- Valid Assignment
					valid_reg <= '1' and c_entity_data_valid_0 and d_entity_data_valid_0 and e_entity_data_valid_0 and counter_entity_data_valid_0;
                    -- Evaluator Done assignment
					upd_and_pe_done <= '1' and (not a_en or a_upd_done) and (not b_en or b_upd_done) and (not val_en or val_upd_done) and (not c_en or c_pe_done) and (not d_en or d_pe_done) and (not e_en or e_pe_done) and (not counter_en or counter_pe_done);
					evaluator_done <= upd_and_pe_done and (not c_en or c_eval_done) and (not d_en or d_eval_done) and (not e_en or e_eval_done) and (not counter_en or counter_eval_done);
                end if;
            else
                upd_and_pe_done <= '0';
				a_upd <= '0';
				b_upd <= '0';
				val_upd <= '0';
				c_pe <= '0';
				c_eval <= '0';
				d_pe <= '0';
				d_eval <= '0';
				e_pe <= '0';
				e_eval <= '0';
				counter_pe <= '0';
				counter_eval <= '0';
            end if;
        end if;
    end process;

    -- output port assignment
	c <= c_entity_data_0;
	d <= d_entity_data_0;
	e <= e_entity_data_0;
	counter <= counter_entity_data_0;
    valid <= valid_reg;
    done <= evaluator_done;

end mixed;