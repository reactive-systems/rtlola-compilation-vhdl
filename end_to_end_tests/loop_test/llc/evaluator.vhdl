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
		b : in signed(7 downto 0);
		b_en : in std_logic;
		c_en : in std_logic;
		d_en : in std_logic;
		e_en : in std_logic;
		f_en : in std_logic;
		g_en : in std_logic;
		time_stream_en : in std_logic;
		c : out signed(7 downto 0);
		d : out signed(7 downto 0);
		e : out signed(7 downto 0);
		f : out signed(7 downto 0);
		g : out signed(7 downto 0);
		time_stream : out signed(7 downto 0);
        done : out std_logic;
        valid : out std_logic
    );
end evaluator;

--* Specification:
--* input a : Int8
--* input b : Int8
--* output c := ((a + b) + d.offset(by: neg1).defaults(to: 3))
--* output d := (b + e.offset(by: neg2).defaults(to: 4))
--* output e := (a.offset(by: neg1).defaults(to: 1) + c)
--* output f := (a + a.offset(by: neg1).defaults(to: 0))
--* output g := (b + f)
--* output time_stream := a.hold().defaults(to: 6)


architecture mixed of evaluator is

    -- Component Declaration
	--* input a : Int8
    component a_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in signed(7 downto 0);
		    data_out : out signed8_array(1 downto 0);
		    data_valid_out : out bit_array(1 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* input b : Int8
    component b_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in signed(7 downto 0);
		    data_out : out signed8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* output c := ((a + b) + d.offset(by: neg1).defaults(to: 3))
    component c_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in signed(7 downto 0);
			a_data_valid_0 : in std_logic;
			b_0 : in signed(7 downto 0);
			b_data_valid_0 : in std_logic;
			d_neg1 : in signed(7 downto 0);
			d_data_valid_neg1 : in std_logic;
		    data_out : out signed8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output d := (b + e.offset(by: neg2).defaults(to: 4))
    component d_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			b_0 : in signed(7 downto 0);
			b_data_valid_0 : in std_logic;
			e_neg2 : in signed(7 downto 0);
			e_data_valid_neg2 : in std_logic;
		    data_out : out signed8_array(1 downto 0);
		    data_valid_out : out bit_array(1 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output e := (a.offset(by: neg1).defaults(to: 1) + c)
    component e_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_neg1 : in signed(7 downto 0);
			a_data_valid_neg1 : in std_logic;
			c_0 : in signed(7 downto 0);
			c_data_valid_0 : in std_logic;
		    data_out : out signed8_array(2 downto 0);
		    data_valid_out : out bit_array(2 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output f := (a + a.offset(by: neg1).defaults(to: 0))
    component f_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in signed(7 downto 0);
			a_data_valid_0 : in std_logic;
			a_neg1 : in signed(7 downto 0);
			a_data_valid_neg1 : in std_logic;
		    data_out : out signed8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output g := (b + f)
    component g_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			b_0 : in signed(7 downto 0);
			b_data_valid_0 : in std_logic;
			f_0 : in signed(7 downto 0);
			f_data_valid_0 : in std_logic;
		    data_out : out signed8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output time_stream := a.hold().defaults(to: 6)
    component time_stream_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in signed(7 downto 0);
			a_data_valid_0 : in std_logic;
		    data_out : out signed8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;


    -- Internal Signal Declarations
	signal a_upd : std_logic;
	signal a_upd_done : std_logic;
	signal a_entity_data_0 : signed(7 downto 0);
	signal a_entity_data_valid_0 : std_logic;
	signal a_entity_data_1 : signed(7 downto 0);
	signal a_entity_data_valid_1 : std_logic;
	signal b_upd : std_logic;
	signal b_upd_done : std_logic;
	signal b_entity_data_0 : signed(7 downto 0);
	signal b_entity_data_valid_0 : std_logic;
	signal c_pe : std_logic;
	signal c_eval : std_logic;
	signal c_pe_done : std_logic;
	signal c_eval_done : std_logic;
	signal c_entity_data_0 : signed(7 downto 0);
	signal c_entity_data_valid_0 : std_logic;
	signal d_pe : std_logic;
	signal d_eval : std_logic;
	signal d_pe_done : std_logic;
	signal d_eval_done : std_logic;
	signal d_entity_data_0 : signed(7 downto 0);
	signal d_entity_data_valid_0 : std_logic;
	signal d_entity_data_1 : signed(7 downto 0);
	signal d_entity_data_valid_1 : std_logic;
	signal e_pe : std_logic;
	signal e_eval : std_logic;
	signal e_pe_done : std_logic;
	signal e_eval_done : std_logic;
	signal e_entity_data_0 : signed(7 downto 0);
	signal e_entity_data_valid_0 : std_logic;
	signal e_entity_data_1 : signed(7 downto 0);
	signal e_entity_data_valid_1 : std_logic;
	signal e_entity_data_2 : signed(7 downto 0);
	signal e_entity_data_valid_2 : std_logic;
	signal f_pe : std_logic;
	signal f_eval : std_logic;
	signal f_pe_done : std_logic;
	signal f_eval_done : std_logic;
	signal f_entity_data_0 : signed(7 downto 0);
	signal f_entity_data_valid_0 : std_logic;
	signal g_pe : std_logic;
	signal g_eval : std_logic;
	signal g_pe_done : std_logic;
	signal g_eval_done : std_logic;
	signal g_entity_data_0 : signed(7 downto 0);
	signal g_entity_data_valid_0 : std_logic;
	signal time_stream_pe : std_logic;
	signal time_stream_eval : std_logic;
	signal time_stream_pe_done : std_logic;
	signal time_stream_eval_done : std_logic;
	signal time_stream_entity_data_0 : signed(7 downto 0);
	signal time_stream_entity_data_valid_0 : std_logic;

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
			data_out(1) => a_entity_data_1,
			data_valid_out(0) => a_entity_data_valid_0,
			data_valid_out(1) => a_entity_data_valid_1,
            done_out => a_upd_done
         );

	--* input b : Int8
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

	--* output c := ((a + b) + d.offset(by: neg1).defaults(to: 3))
    c_entity_instance: c_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => c_pe,
            eval => c_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			d_neg1 => d_entity_data_1,
			d_data_valid_neg1 => d_entity_data_valid_1,
			data_out(0) => c_entity_data_0,
			data_valid_out(0) => c_entity_data_valid_0,
            pe_done_out => c_pe_done,
            eval_done_out => c_eval_done
        );

	--* output d := (b + e.offset(by: neg2).defaults(to: 4))
    d_entity_instance: d_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => d_pe,
            eval => d_eval,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			e_neg2 => e_entity_data_2,
			e_data_valid_neg2 => e_entity_data_valid_2,
			data_out(0) => d_entity_data_0,
			data_out(1) => d_entity_data_1,
			data_valid_out(0) => d_entity_data_valid_0,
			data_valid_out(1) => d_entity_data_valid_1,
            pe_done_out => d_pe_done,
            eval_done_out => d_eval_done
        );

	--* output e := (a.offset(by: neg1).defaults(to: 1) + c)
    e_entity_instance: e_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => e_pe,
            eval => e_eval,
			a_neg1 => a_entity_data_1,
			a_data_valid_neg1 => a_entity_data_valid_1,
			c_0 => c_entity_data_0,
			c_data_valid_0 => c_entity_data_valid_0,
			data_out(0) => e_entity_data_0,
			data_out(1) => e_entity_data_1,
			data_out(2) => e_entity_data_2,
			data_valid_out(0) => e_entity_data_valid_0,
			data_valid_out(1) => e_entity_data_valid_1,
			data_valid_out(2) => e_entity_data_valid_2,
            pe_done_out => e_pe_done,
            eval_done_out => e_eval_done
        );

	--* output f := (a + a.offset(by: neg1).defaults(to: 0))
    f_entity_instance: f_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => f_pe,
            eval => f_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			a_neg1 => a_entity_data_1,
			a_data_valid_neg1 => a_entity_data_valid_1,
			data_out(0) => f_entity_data_0,
			data_valid_out(0) => f_entity_data_valid_0,
            pe_done_out => f_pe_done,
            eval_done_out => f_eval_done
        );

	--* output g := (b + f)
    g_entity_instance: g_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => g_pe,
            eval => g_eval,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			f_0 => f_entity_data_0,
			f_data_valid_0 => f_entity_data_valid_0,
			data_out(0) => g_entity_data_0,
			data_valid_out(0) => g_entity_data_valid_0,
            pe_done_out => g_pe_done,
            eval_done_out => g_eval_done
        );

	--* output time_stream := a.hold().defaults(to: 6)
    time_stream_entity_instance: time_stream_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => time_stream_pe,
            eval => time_stream_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			data_out(0) => time_stream_entity_data_0,
			data_valid_out(0) => time_stream_entity_data_valid_0,
            pe_done_out => time_stream_pe_done,
            eval_done_out => time_stream_eval_done
        );


    process(clk, rst) begin
        if rst = '1' then
            -- Reset Phase
            valid_reg <= '0';
				a_upd <= '0';
				b_upd <= '0';
				c_pe <= '0';
				c_eval <= '0';
				d_pe <= '0';
				d_eval <= '0';
				e_pe <= '0';
				e_eval <= '0';
				f_pe <= '0';
				f_eval <= '0';
				g_pe <= '0';
				g_eval <= '0';
				time_stream_pe <= '0';
				time_stream_eval <= '0';
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
					a_upd <= a_en;
					b_upd <= b_en;
                    -- Pseudo Evaluation Phase
                    --* Output Streams in Specification 
					--* - c
					--* - d
					--* - e
					--* - f
					--* - g
					--* - time_stream
					c_pe <= c_en;
					d_pe <= d_en;
					e_pe <= e_en;
					f_pe <= f_en;
					g_pe <= g_en;
					time_stream_pe <= time_stream_en;
                    -- Evict Phase
                    --* Sliding Windows in Specification 
                    upd_and_pe_done <= '1';
                    evaluator_done <= '0';
                else
                    -- Eval Phase
					--* output c := ((a + b) + d.offset(by: neg1).defaults(to: 3))
					--* Evaluation Phase of Output Stream c is Influenced by No Lookup
					c_eval <= c_en and upd_and_pe_done;
					--* output d := (b + e.offset(by: neg2).defaults(to: 4))
					--* Evaluation Phase of Output Stream d is Influenced by No Lookup
					d_eval <= d_en and upd_and_pe_done;
					--* output e := (a.offset(by: neg1).defaults(to: 1) + c)
					--* Evaluation Phase of Output Stream e is Influenced by the following Lookups: 
					--* - Synchronous Lookup: c
					e_eval <= e_en and upd_and_pe_done and c_eval_done;
					--* output f := (a + a.offset(by: neg1).defaults(to: 0))
					--* Evaluation Phase of Output Stream f is Influenced by No Lookup
					f_eval <= f_en and upd_and_pe_done;
					--* output g := (b + f)
					--* Evaluation Phase of Output Stream g is Influenced by the following Lookups: 
					--* - Synchronous Lookup: f
					g_eval <= g_en and upd_and_pe_done and f_eval_done;
					--* output time_stream := a.hold().defaults(to: 6)
					--* Evaluation Phase of Output Stream time_stream is Influenced by No Lookup
					time_stream_eval <= time_stream_en and upd_and_pe_done;
                    -- SW Update Phase
                    -- SW Request Phase
                    -- Valid Assignment
					valid_reg <= '1' and c_entity_data_valid_0 and d_entity_data_valid_0 and e_entity_data_valid_0 and f_entity_data_valid_0 and g_entity_data_valid_0 and time_stream_entity_data_valid_0;
                    -- Evaluator Done assignment
					upd_and_pe_done <= '1' and (not a_en or a_upd_done) and (not b_en or b_upd_done) and (not c_en or c_pe_done) and (not d_en or d_pe_done) and (not e_en or e_pe_done) and (not f_en or f_pe_done) and (not g_en or g_pe_done) and (not time_stream_en or time_stream_pe_done);
					evaluator_done <= upd_and_pe_done and (not c_en or c_eval_done) and (not d_en or d_eval_done) and (not e_en or e_eval_done) and (not f_en or f_eval_done) and (not g_en or g_eval_done) and (not time_stream_en or time_stream_eval_done);
                end if;
            else
                upd_and_pe_done <= '0';
				a_upd <= '0';
				b_upd <= '0';
				c_pe <= '0';
				c_eval <= '0';
				d_pe <= '0';
				d_eval <= '0';
				e_pe <= '0';
				e_eval <= '0';
				f_pe <= '0';
				f_eval <= '0';
				g_pe <= '0';
				g_eval <= '0';
				time_stream_pe <= '0';
				time_stream_eval <= '0';
            end if;
        end if;
    end process;

    -- output port assignment
	c <= c_entity_data_0;
	d <= d_entity_data_0;
	e <= e_entity_data_0;
	f <= f_entity_data_0;
	g <= g_entity_data_0;
	time_stream <= time_stream_entity_data_0;
    valid <= valid_reg;
    done <= evaluator_done;

end mixed;