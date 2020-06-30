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
		b_en : in std_logic;
		c_en : in std_logic;
		d_en : in std_logic;
		b : out signed(31 downto 0);
		c : out signed(31 downto 0);
		d : out signed(31 downto 0);
        done : out std_logic;
        valid : out std_logic
    );
end evaluator;

--* Specification:
--* input a : Int32
--* output b := a.hold().defaults(to: 10)
--* output c := a.hold().defaults(to: 10)
--* output d := (b + c)


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

	--* output b := a.hold().defaults(to: 10)
    component b_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in signed(31 downto 0);
			a_data_valid_0 : in std_logic;
		    data_out : out signed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output c := a.hold().defaults(to: 10)
    component c_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in signed(31 downto 0);
			a_data_valid_0 : in std_logic;
		    data_out : out signed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output d := (b + c)
    component d_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			b_0 : in signed(31 downto 0);
			b_data_valid_0 : in std_logic;
			c_0 : in signed(31 downto 0);
			c_data_valid_0 : in std_logic;
		    data_out : out signed32_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;


    -- Internal Signal Declarations
	signal a_upd : std_logic;
	signal a_upd_done : std_logic;
	signal a_entity_data_0 : signed(31 downto 0);
	signal a_entity_data_valid_0 : std_logic;
	signal b_pe : std_logic;
	signal b_eval : std_logic;
	signal b_pe_done : std_logic;
	signal b_eval_done : std_logic;
	signal b_entity_data_0 : signed(31 downto 0);
	signal b_entity_data_valid_0 : std_logic;
	signal c_pe : std_logic;
	signal c_eval : std_logic;
	signal c_pe_done : std_logic;
	signal c_eval_done : std_logic;
	signal c_entity_data_0 : signed(31 downto 0);
	signal c_entity_data_valid_0 : std_logic;
	signal d_pe : std_logic;
	signal d_eval : std_logic;
	signal d_pe_done : std_logic;
	signal d_eval_done : std_logic;
	signal d_entity_data_0 : signed(31 downto 0);
	signal d_entity_data_valid_0 : std_logic;

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

	--* output b := a.hold().defaults(to: 10)
    b_entity_instance: b_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => b_pe,
            eval => b_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			data_out(0) => b_entity_data_0,
			data_valid_out(0) => b_entity_data_valid_0,
            pe_done_out => b_pe_done,
            eval_done_out => b_eval_done
        );

	--* output c := a.hold().defaults(to: 10)
    c_entity_instance: c_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => c_pe,
            eval => c_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			data_out(0) => c_entity_data_0,
			data_valid_out(0) => c_entity_data_valid_0,
            pe_done_out => c_pe_done,
            eval_done_out => c_eval_done
        );

	--* output d := (b + c)
    d_entity_instance: d_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => d_pe,
            eval => d_eval,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			c_0 => c_entity_data_0,
			c_data_valid_0 => c_entity_data_valid_0,
			data_out(0) => d_entity_data_0,
			data_valid_out(0) => d_entity_data_valid_0,
            pe_done_out => d_pe_done,
            eval_done_out => d_eval_done
        );


    process(clk, rst) begin
        if rst = '1' then
            -- Reset Phase
            valid_reg <= '0';
				a_upd <= '0';
				b_pe <= '0';
				b_eval <= '0';
				c_pe <= '0';
				c_eval <= '0';
				d_pe <= '0';
				d_eval <= '0';
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
					--* - b
					--* - c
					--* - d
					b_pe <= b_en;
					c_pe <= c_en;
					d_pe <= d_en;
                    -- Evict Phase
                    --* Sliding Windows in Specification 
                    upd_and_pe_done <= '1';
                    evaluator_done <= '0';
                else
                    -- Eval Phase
					--* output b := a.hold().defaults(to: 10)
					--* Evaluation Phase of Output Stream b is Influenced by No Lookup
					b_eval <= b_en and upd_and_pe_done;
					--* output c := a.hold().defaults(to: 10)
					--* Evaluation Phase of Output Stream c is Influenced by No Lookup
					c_eval <= c_en and upd_and_pe_done;
					--* output d := (b + c)
					--* Evaluation Phase of Output Stream d is Influenced by the following Lookups: 
					--* - Synchronous Lookup: b
					--* - Synchronous Lookup: c
					d_eval <= d_en and upd_and_pe_done and b_eval_done and c_eval_done;
                    -- SW Update Phase
                    -- SW Request Phase
                    -- Valid Assignment
					valid_reg <= '1' and b_entity_data_valid_0 and c_entity_data_valid_0 and d_entity_data_valid_0;
                    -- Evaluator Done assignment
					upd_and_pe_done <= '1' and (not a_en or a_upd_done) and (not b_en or b_pe_done) and (not c_en or c_pe_done) and (not d_en or d_pe_done);
					evaluator_done <= upd_and_pe_done and (not b_en or b_eval_done) and (not c_en or c_eval_done) and (not d_en or d_eval_done);
                end if;
            else
                upd_and_pe_done <= '0';
				a_upd <= '0';
				b_pe <= '0';
				b_eval <= '0';
				c_pe <= '0';
				c_eval <= '0';
				d_pe <= '0';
				d_eval <= '0';
            end if;
        end if;
    end process;

    -- output port assignment
	b <= b_entity_data_0;
	c <= c_entity_data_0;
	d <= d_entity_data_0;
    valid <= valid_reg;
    done <= evaluator_done;

end mixed;