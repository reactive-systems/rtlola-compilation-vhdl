library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity evaluator is
    port (
        clk, input_clk, rst : in std_logic;
        input_time : in unsigned(63 downto 0);
		a : in std_logic;
		a_en : in std_logic;
		b : in std_logic;
		b_en : in std_logic;
		ID : in signed(7 downto 0);
		ID_en : in std_logic;
		eq_en : in std_logic;
		lt_en : in std_logic;
		le_en : in std_logic;
		gt_en : in std_logic;
		ge_en : in std_logic;
		neq_en : in std_logic;
		not_a_en : in std_logic;
		a_and_b_en : in std_logic;
		a_or_b_en : in std_logic;
		a_impl_b_en : in std_logic;
		a_equiv_b_en : in std_logic;
		a_xor_b_en : in std_logic;
		true_const_en : in std_logic;
		time_stream_en : in std_logic;
		eq : out std_logic;
		lt : out std_logic;
		le : out std_logic;
		gt : out std_logic;
		ge : out std_logic;
		neq : out std_logic;
		not_a : out std_logic;
		a_and_b : out std_logic;
		a_or_b : out std_logic;
		a_impl_b : out std_logic;
		a_equiv_b : out std_logic;
		a_xor_b : out std_logic;
		true_const : out std_logic;
		time_stream : out signed(7 downto 0);
        done : out std_logic;
        valid : out std_logic
    );
end evaluator;

--* Specification:
--* input a : Bool
--* input b : Bool
--* input ID : Int8
--* output eq := (ID = 5)
--* output lt := (ID < 5)
--* output le := (ID <= 5)
--* output gt := (ID > 5)
--* output ge := (ID >= 5)
--* output neq := (lt or gt)
--* output not_a := (not a)
--* output a_and_b := (a and b)
--* output a_or_b := (a or b)
--* output a_impl_b := ((not a) or b)
--* output a_equiv_b := (a_impl_b and ((not b) or a))
--* output a_xor_b := (not a_equiv_b)
--* output true_const := ((lt or gt) or eq)
--* output time_stream := ID.hold().defaults(to: 0)


architecture mixed of evaluator is

    -- Component Declaration
	--* input a : Bool
    component a_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* input b : Bool
    component b_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* input ID : Int8
    component ID_input_stream_entity
	    port (
		    clk, upd, rst : in std_logic;
		    data_in : in signed(7 downto 0);
		    data_out : out signed8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    done_out : out std_logic
	    );
    end component;

	--* output eq := (ID = 5)
    component eq_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			ID_0 : in signed(7 downto 0);
			ID_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output lt := (ID < 5)
    component lt_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			ID_0 : in signed(7 downto 0);
			ID_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output le := (ID <= 5)
    component le_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			ID_0 : in signed(7 downto 0);
			ID_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output gt := (ID > 5)
    component gt_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			ID_0 : in signed(7 downto 0);
			ID_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output ge := (ID >= 5)
    component ge_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			ID_0 : in signed(7 downto 0);
			ID_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output neq := (lt or gt)
    component neq_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			lt_0 : in std_logic;
			lt_data_valid_0 : in std_logic;
			gt_0 : in std_logic;
			gt_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output not_a := (not a)
    component not_a_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in std_logic;
			a_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output a_and_b := (a and b)
    component a_and_b_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in std_logic;
			a_data_valid_0 : in std_logic;
			b_0 : in std_logic;
			b_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output a_or_b := (a or b)
    component a_or_b_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in std_logic;
			a_data_valid_0 : in std_logic;
			b_0 : in std_logic;
			b_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output a_impl_b := ((not a) or b)
    component a_impl_b_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in std_logic;
			a_data_valid_0 : in std_logic;
			b_0 : in std_logic;
			b_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output a_equiv_b := (a_impl_b and ((not b) or a))
    component a_equiv_b_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_0 : in std_logic;
			a_data_valid_0 : in std_logic;
			b_0 : in std_logic;
			b_data_valid_0 : in std_logic;
			a_impl_b_0 : in std_logic;
			a_impl_b_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output a_xor_b := (not a_equiv_b)
    component a_xor_b_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			a_equiv_b_0 : in std_logic;
			a_equiv_b_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output true_const := ((lt or gt) or eq)
    component true_const_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			eq_0 : in std_logic;
			eq_data_valid_0 : in std_logic;
			lt_0 : in std_logic;
			lt_data_valid_0 : in std_logic;
			gt_0 : in std_logic;
			gt_data_valid_0 : in std_logic;
		    data_out : out bit_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;

	--* output time_stream := ID.hold().defaults(to: 0)
    component time_stream_output_stream_entity
	    port (
		    clk, pe, eval, rst : in std_logic;
			ID_0 : in signed(7 downto 0);
			ID_data_valid_0 : in std_logic;
		    data_out : out signed8_array(0 downto 0);
		    data_valid_out : out bit_array(0 downto 0);
		    pe_done_out : out std_logic;
		    eval_done_out : out std_logic
	    );
    end component;


    -- Internal Signal Declarations
	signal a_upd : std_logic;
	signal a_upd_done : std_logic;
	signal a_entity_data_0 : std_logic;
	signal a_entity_data_valid_0 : std_logic;
	signal b_upd : std_logic;
	signal b_upd_done : std_logic;
	signal b_entity_data_0 : std_logic;
	signal b_entity_data_valid_0 : std_logic;
	signal ID_upd : std_logic;
	signal ID_upd_done : std_logic;
	signal ID_entity_data_0 : signed(7 downto 0);
	signal ID_entity_data_valid_0 : std_logic;
	signal eq_pe : std_logic;
	signal eq_eval : std_logic;
	signal eq_pe_done : std_logic;
	signal eq_eval_done : std_logic;
	signal eq_entity_data_0 : std_logic;
	signal eq_entity_data_valid_0 : std_logic;
	signal lt_pe : std_logic;
	signal lt_eval : std_logic;
	signal lt_pe_done : std_logic;
	signal lt_eval_done : std_logic;
	signal lt_entity_data_0 : std_logic;
	signal lt_entity_data_valid_0 : std_logic;
	signal le_pe : std_logic;
	signal le_eval : std_logic;
	signal le_pe_done : std_logic;
	signal le_eval_done : std_logic;
	signal le_entity_data_0 : std_logic;
	signal le_entity_data_valid_0 : std_logic;
	signal gt_pe : std_logic;
	signal gt_eval : std_logic;
	signal gt_pe_done : std_logic;
	signal gt_eval_done : std_logic;
	signal gt_entity_data_0 : std_logic;
	signal gt_entity_data_valid_0 : std_logic;
	signal ge_pe : std_logic;
	signal ge_eval : std_logic;
	signal ge_pe_done : std_logic;
	signal ge_eval_done : std_logic;
	signal ge_entity_data_0 : std_logic;
	signal ge_entity_data_valid_0 : std_logic;
	signal neq_pe : std_logic;
	signal neq_eval : std_logic;
	signal neq_pe_done : std_logic;
	signal neq_eval_done : std_logic;
	signal neq_entity_data_0 : std_logic;
	signal neq_entity_data_valid_0 : std_logic;
	signal not_a_pe : std_logic;
	signal not_a_eval : std_logic;
	signal not_a_pe_done : std_logic;
	signal not_a_eval_done : std_logic;
	signal not_a_entity_data_0 : std_logic;
	signal not_a_entity_data_valid_0 : std_logic;
	signal a_and_b_pe : std_logic;
	signal a_and_b_eval : std_logic;
	signal a_and_b_pe_done : std_logic;
	signal a_and_b_eval_done : std_logic;
	signal a_and_b_entity_data_0 : std_logic;
	signal a_and_b_entity_data_valid_0 : std_logic;
	signal a_or_b_pe : std_logic;
	signal a_or_b_eval : std_logic;
	signal a_or_b_pe_done : std_logic;
	signal a_or_b_eval_done : std_logic;
	signal a_or_b_entity_data_0 : std_logic;
	signal a_or_b_entity_data_valid_0 : std_logic;
	signal a_impl_b_pe : std_logic;
	signal a_impl_b_eval : std_logic;
	signal a_impl_b_pe_done : std_logic;
	signal a_impl_b_eval_done : std_logic;
	signal a_impl_b_entity_data_0 : std_logic;
	signal a_impl_b_entity_data_valid_0 : std_logic;
	signal a_equiv_b_pe : std_logic;
	signal a_equiv_b_eval : std_logic;
	signal a_equiv_b_pe_done : std_logic;
	signal a_equiv_b_eval_done : std_logic;
	signal a_equiv_b_entity_data_0 : std_logic;
	signal a_equiv_b_entity_data_valid_0 : std_logic;
	signal a_xor_b_pe : std_logic;
	signal a_xor_b_eval : std_logic;
	signal a_xor_b_pe_done : std_logic;
	signal a_xor_b_eval_done : std_logic;
	signal a_xor_b_entity_data_0 : std_logic;
	signal a_xor_b_entity_data_valid_0 : std_logic;
	signal true_const_pe : std_logic;
	signal true_const_eval : std_logic;
	signal true_const_pe_done : std_logic;
	signal true_const_eval_done : std_logic;
	signal true_const_entity_data_0 : std_logic;
	signal true_const_entity_data_valid_0 : std_logic;
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
	--* input a : Bool
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

	--* input b : Bool
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

	--* input ID : Int8
    ID_entity_instance: ID_input_stream_entity
        port map (
            clk => clk,
            rst => rst,
            upd => ID_upd,
            data_in => ID,
			data_out(0) => ID_entity_data_0,
			data_valid_out(0) => ID_entity_data_valid_0,
            done_out => ID_upd_done
         );

	--* output eq := (ID = 5)
    eq_entity_instance: eq_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => eq_pe,
            eval => eq_eval,
			ID_0 => ID_entity_data_0,
			ID_data_valid_0 => ID_entity_data_valid_0,
			data_out(0) => eq_entity_data_0,
			data_valid_out(0) => eq_entity_data_valid_0,
            pe_done_out => eq_pe_done,
            eval_done_out => eq_eval_done
        );

	--* output lt := (ID < 5)
    lt_entity_instance: lt_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => lt_pe,
            eval => lt_eval,
			ID_0 => ID_entity_data_0,
			ID_data_valid_0 => ID_entity_data_valid_0,
			data_out(0) => lt_entity_data_0,
			data_valid_out(0) => lt_entity_data_valid_0,
            pe_done_out => lt_pe_done,
            eval_done_out => lt_eval_done
        );

	--* output le := (ID <= 5)
    le_entity_instance: le_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => le_pe,
            eval => le_eval,
			ID_0 => ID_entity_data_0,
			ID_data_valid_0 => ID_entity_data_valid_0,
			data_out(0) => le_entity_data_0,
			data_valid_out(0) => le_entity_data_valid_0,
            pe_done_out => le_pe_done,
            eval_done_out => le_eval_done
        );

	--* output gt := (ID > 5)
    gt_entity_instance: gt_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => gt_pe,
            eval => gt_eval,
			ID_0 => ID_entity_data_0,
			ID_data_valid_0 => ID_entity_data_valid_0,
			data_out(0) => gt_entity_data_0,
			data_valid_out(0) => gt_entity_data_valid_0,
            pe_done_out => gt_pe_done,
            eval_done_out => gt_eval_done
        );

	--* output ge := (ID >= 5)
    ge_entity_instance: ge_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => ge_pe,
            eval => ge_eval,
			ID_0 => ID_entity_data_0,
			ID_data_valid_0 => ID_entity_data_valid_0,
			data_out(0) => ge_entity_data_0,
			data_valid_out(0) => ge_entity_data_valid_0,
            pe_done_out => ge_pe_done,
            eval_done_out => ge_eval_done
        );

	--* output neq := (lt or gt)
    neq_entity_instance: neq_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => neq_pe,
            eval => neq_eval,
			lt_0 => lt_entity_data_0,
			lt_data_valid_0 => lt_entity_data_valid_0,
			gt_0 => gt_entity_data_0,
			gt_data_valid_0 => gt_entity_data_valid_0,
			data_out(0) => neq_entity_data_0,
			data_valid_out(0) => neq_entity_data_valid_0,
            pe_done_out => neq_pe_done,
            eval_done_out => neq_eval_done
        );

	--* output not_a := (not a)
    not_a_entity_instance: not_a_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => not_a_pe,
            eval => not_a_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			data_out(0) => not_a_entity_data_0,
			data_valid_out(0) => not_a_entity_data_valid_0,
            pe_done_out => not_a_pe_done,
            eval_done_out => not_a_eval_done
        );

	--* output a_and_b := (a and b)
    a_and_b_entity_instance: a_and_b_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => a_and_b_pe,
            eval => a_and_b_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			data_out(0) => a_and_b_entity_data_0,
			data_valid_out(0) => a_and_b_entity_data_valid_0,
            pe_done_out => a_and_b_pe_done,
            eval_done_out => a_and_b_eval_done
        );

	--* output a_or_b := (a or b)
    a_or_b_entity_instance: a_or_b_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => a_or_b_pe,
            eval => a_or_b_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			data_out(0) => a_or_b_entity_data_0,
			data_valid_out(0) => a_or_b_entity_data_valid_0,
            pe_done_out => a_or_b_pe_done,
            eval_done_out => a_or_b_eval_done
        );

	--* output a_impl_b := ((not a) or b)
    a_impl_b_entity_instance: a_impl_b_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => a_impl_b_pe,
            eval => a_impl_b_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			data_out(0) => a_impl_b_entity_data_0,
			data_valid_out(0) => a_impl_b_entity_data_valid_0,
            pe_done_out => a_impl_b_pe_done,
            eval_done_out => a_impl_b_eval_done
        );

	--* output a_equiv_b := (a_impl_b and ((not b) or a))
    a_equiv_b_entity_instance: a_equiv_b_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => a_equiv_b_pe,
            eval => a_equiv_b_eval,
			a_0 => a_entity_data_0,
			a_data_valid_0 => a_entity_data_valid_0,
			b_0 => b_entity_data_0,
			b_data_valid_0 => b_entity_data_valid_0,
			a_impl_b_0 => a_impl_b_entity_data_0,
			a_impl_b_data_valid_0 => a_impl_b_entity_data_valid_0,
			data_out(0) => a_equiv_b_entity_data_0,
			data_valid_out(0) => a_equiv_b_entity_data_valid_0,
            pe_done_out => a_equiv_b_pe_done,
            eval_done_out => a_equiv_b_eval_done
        );

	--* output a_xor_b := (not a_equiv_b)
    a_xor_b_entity_instance: a_xor_b_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => a_xor_b_pe,
            eval => a_xor_b_eval,
			a_equiv_b_0 => a_equiv_b_entity_data_0,
			a_equiv_b_data_valid_0 => a_equiv_b_entity_data_valid_0,
			data_out(0) => a_xor_b_entity_data_0,
			data_valid_out(0) => a_xor_b_entity_data_valid_0,
            pe_done_out => a_xor_b_pe_done,
            eval_done_out => a_xor_b_eval_done
        );

	--* output true_const := ((lt or gt) or eq)
    true_const_entity_instance: true_const_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => true_const_pe,
            eval => true_const_eval,
			eq_0 => eq_entity_data_0,
			eq_data_valid_0 => eq_entity_data_valid_0,
			lt_0 => lt_entity_data_0,
			lt_data_valid_0 => lt_entity_data_valid_0,
			gt_0 => gt_entity_data_0,
			gt_data_valid_0 => gt_entity_data_valid_0,
			data_out(0) => true_const_entity_data_0,
			data_valid_out(0) => true_const_entity_data_valid_0,
            pe_done_out => true_const_pe_done,
            eval_done_out => true_const_eval_done
        );

	--* output time_stream := ID.hold().defaults(to: 0)
    time_stream_entity_instance: time_stream_output_stream_entity
        port map (
            clk => clk,
            rst => rst,
            pe => time_stream_pe,
            eval => time_stream_eval,
			ID_0 => ID_entity_data_0,
			ID_data_valid_0 => ID_entity_data_valid_0,
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
				ID_upd <= '0';
				eq_pe <= '0';
				eq_eval <= '0';
				lt_pe <= '0';
				lt_eval <= '0';
				le_pe <= '0';
				le_eval <= '0';
				gt_pe <= '0';
				gt_eval <= '0';
				ge_pe <= '0';
				ge_eval <= '0';
				neq_pe <= '0';
				neq_eval <= '0';
				not_a_pe <= '0';
				not_a_eval <= '0';
				a_and_b_pe <= '0';
				a_and_b_eval <= '0';
				a_or_b_pe <= '0';
				a_or_b_eval <= '0';
				a_impl_b_pe <= '0';
				a_impl_b_eval <= '0';
				a_equiv_b_pe <= '0';
				a_equiv_b_eval <= '0';
				a_xor_b_pe <= '0';
				a_xor_b_eval <= '0';
				true_const_pe <= '0';
				true_const_eval <= '0';
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
					--* - ID 
					a_upd <= a_en;
					b_upd <= b_en;
					ID_upd <= ID_en;
                    -- Pseudo Evaluation Phase
                    --* Output Streams in Specification 
					--* - eq
					--* - lt
					--* - le
					--* - gt
					--* - ge
					--* - neq
					--* - not_a
					--* - a_and_b
					--* - a_or_b
					--* - a_impl_b
					--* - a_equiv_b
					--* - a_xor_b
					--* - true_const
					--* - time_stream
					eq_pe <= eq_en;
					lt_pe <= lt_en;
					le_pe <= le_en;
					gt_pe <= gt_en;
					ge_pe <= ge_en;
					neq_pe <= neq_en;
					not_a_pe <= not_a_en;
					a_and_b_pe <= a_and_b_en;
					a_or_b_pe <= a_or_b_en;
					a_impl_b_pe <= a_impl_b_en;
					a_equiv_b_pe <= a_equiv_b_en;
					a_xor_b_pe <= a_xor_b_en;
					true_const_pe <= true_const_en;
					time_stream_pe <= time_stream_en;
                    -- Evict Phase
                    --* Sliding Windows in Specification 
                    upd_and_pe_done <= '1';
                    evaluator_done <= '0';
                else
                    -- Eval Phase
					--* output eq := (ID = 5)
					--* Evaluation Phase of Output Stream eq is Influenced by No Lookup
					eq_eval <= eq_en and upd_and_pe_done;
					--* output lt := (ID < 5)
					--* Evaluation Phase of Output Stream lt is Influenced by No Lookup
					lt_eval <= lt_en and upd_and_pe_done;
					--* output le := (ID <= 5)
					--* Evaluation Phase of Output Stream le is Influenced by No Lookup
					le_eval <= le_en and upd_and_pe_done;
					--* output gt := (ID > 5)
					--* Evaluation Phase of Output Stream gt is Influenced by No Lookup
					gt_eval <= gt_en and upd_and_pe_done;
					--* output ge := (ID >= 5)
					--* Evaluation Phase of Output Stream ge is Influenced by No Lookup
					ge_eval <= ge_en and upd_and_pe_done;
					--* output neq := (lt or gt)
					--* Evaluation Phase of Output Stream neq is Influenced by the following Lookups: 
					--* - Synchronous Lookup: lt
					--* - Synchronous Lookup: gt
					neq_eval <= neq_en and upd_and_pe_done and lt_eval_done and gt_eval_done;
					--* output not_a := (not a)
					--* Evaluation Phase of Output Stream not_a is Influenced by No Lookup
					not_a_eval <= not_a_en and upd_and_pe_done;
					--* output a_and_b := (a and b)
					--* Evaluation Phase of Output Stream a_and_b is Influenced by No Lookup
					a_and_b_eval <= a_and_b_en and upd_and_pe_done;
					--* output a_or_b := (a or b)
					--* Evaluation Phase of Output Stream a_or_b is Influenced by No Lookup
					a_or_b_eval <= a_or_b_en and upd_and_pe_done;
					--* output a_impl_b := ((not a) or b)
					--* Evaluation Phase of Output Stream a_impl_b is Influenced by No Lookup
					a_impl_b_eval <= a_impl_b_en and upd_and_pe_done;
					--* output a_equiv_b := (a_impl_b and ((not b) or a))
					--* Evaluation Phase of Output Stream a_equiv_b is Influenced by the following Lookups: 
					--* - Synchronous Lookup: a_impl_b
					a_equiv_b_eval <= a_equiv_b_en and upd_and_pe_done and a_impl_b_eval_done;
					--* output a_xor_b := (not a_equiv_b)
					--* Evaluation Phase of Output Stream a_xor_b is Influenced by the following Lookups: 
					--* - Synchronous Lookup: a_equiv_b
					a_xor_b_eval <= a_xor_b_en and upd_and_pe_done and a_equiv_b_eval_done;
					--* output true_const := ((lt or gt) or eq)
					--* Evaluation Phase of Output Stream true_const is Influenced by the following Lookups: 
					--* - Synchronous Lookup: eq
					--* - Synchronous Lookup: lt
					--* - Synchronous Lookup: gt
					true_const_eval <= true_const_en and upd_and_pe_done and eq_eval_done and lt_eval_done and gt_eval_done;
					--* output time_stream := ID.hold().defaults(to: 0)
					--* Evaluation Phase of Output Stream time_stream is Influenced by No Lookup
					time_stream_eval <= time_stream_en and upd_and_pe_done;
                    -- SW Update Phase
                    -- SW Request Phase
                    -- Valid Assignment
					valid_reg <= '1' and eq_entity_data_valid_0 and lt_entity_data_valid_0 and le_entity_data_valid_0 and gt_entity_data_valid_0 and ge_entity_data_valid_0 and neq_entity_data_valid_0 and not_a_entity_data_valid_0 and a_and_b_entity_data_valid_0 and a_or_b_entity_data_valid_0 and a_impl_b_entity_data_valid_0 and a_equiv_b_entity_data_valid_0 and a_xor_b_entity_data_valid_0 and true_const_entity_data_valid_0 and time_stream_entity_data_valid_0;
                    -- Evaluator Done assignment
					upd_and_pe_done <= '1' and (not a_en or a_upd_done) and (not b_en or b_upd_done) and (not ID_en or ID_upd_done) and (not eq_en or eq_pe_done) and (not lt_en or lt_pe_done) and (not le_en or le_pe_done) and (not gt_en or gt_pe_done) and (not ge_en or ge_pe_done) and (not neq_en or neq_pe_done) and (not not_a_en or not_a_pe_done) and (not a_and_b_en or a_and_b_pe_done) and (not a_or_b_en or a_or_b_pe_done) and (not a_impl_b_en or a_impl_b_pe_done) and (not a_equiv_b_en or a_equiv_b_pe_done) and (not a_xor_b_en or a_xor_b_pe_done) and (not true_const_en or true_const_pe_done) and (not time_stream_en or time_stream_pe_done);
					evaluator_done <= upd_and_pe_done and (not eq_en or eq_eval_done) and (not lt_en or lt_eval_done) and (not le_en or le_eval_done) and (not gt_en or gt_eval_done) and (not ge_en or ge_eval_done) and (not neq_en or neq_eval_done) and (not not_a_en or not_a_eval_done) and (not a_and_b_en or a_and_b_eval_done) and (not a_or_b_en or a_or_b_eval_done) and (not a_impl_b_en or a_impl_b_eval_done) and (not a_equiv_b_en or a_equiv_b_eval_done) and (not a_xor_b_en or a_xor_b_eval_done) and (not true_const_en or true_const_eval_done) and (not time_stream_en or time_stream_eval_done);
                end if;
            else
                upd_and_pe_done <= '0';
				a_upd <= '0';
				b_upd <= '0';
				ID_upd <= '0';
				eq_pe <= '0';
				eq_eval <= '0';
				lt_pe <= '0';
				lt_eval <= '0';
				le_pe <= '0';
				le_eval <= '0';
				gt_pe <= '0';
				gt_eval <= '0';
				ge_pe <= '0';
				ge_eval <= '0';
				neq_pe <= '0';
				neq_eval <= '0';
				not_a_pe <= '0';
				not_a_eval <= '0';
				a_and_b_pe <= '0';
				a_and_b_eval <= '0';
				a_or_b_pe <= '0';
				a_or_b_eval <= '0';
				a_impl_b_pe <= '0';
				a_impl_b_eval <= '0';
				a_equiv_b_pe <= '0';
				a_equiv_b_eval <= '0';
				a_xor_b_pe <= '0';
				a_xor_b_eval <= '0';
				true_const_pe <= '0';
				true_const_eval <= '0';
				time_stream_pe <= '0';
				time_stream_eval <= '0';
            end if;
        end if;
    end process;

    -- output port assignment
	eq <= eq_entity_data_0;
	lt <= lt_entity_data_0;
	le <= le_entity_data_0;
	gt <= gt_entity_data_0;
	ge <= ge_entity_data_0;
	neq <= neq_entity_data_0;
	not_a <= not_a_entity_data_0;
	a_and_b <= a_and_b_entity_data_0;
	a_or_b <= a_or_b_entity_data_0;
	a_impl_b <= a_impl_b_entity_data_0;
	a_equiv_b <= a_equiv_b_entity_data_0;
	a_xor_b <= a_xor_b_entity_data_0;
	true_const <= true_const_entity_data_0;
	time_stream <= time_stream_entity_data_0;
    valid <= valid_reg;
    done <= evaluator_done;

end mixed;