library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

entity monitor is
    port (
        clk, tclk, qclk, eclk, rst : in std_logic;
        input_time : in std_logic_vector(63 downto 0);
        offline : in std_logic;
        new_input : in std_logic;
		a_data_in : in std_logic;
		a_data_in_new_input : in std_logic;
		b_data_in : in std_logic;
		b_data_in_new_input : in std_logic;
		ID_data_in : in std_logic_vector(7 downto 0);
		ID_data_in_new_input : in std_logic;
        time_stream : out std_logic_vector(63 downto 0);
		a_stream: out std_logic;
		b_stream: out std_logic;
		ID_stream: out std_logic_vector(7 downto 0);
		eq_stream: out std_logic;
		lt_stream: out std_logic;
		le_stream: out std_logic;
		gt_stream: out std_logic;
		ge_stream: out std_logic;
		neq_stream: out std_logic;
		not_a_stream: out std_logic;
		a_and_b_stream: out std_logic;
		a_or_b_stream: out std_logic;
		a_impl_b_stream: out std_logic;
		a_equiv_b_stream: out std_logic;
		a_xor_b_stream: out std_logic;
		true_const_stream: out std_logic;
		time_stream_stream: out std_logic_vector(7 downto 0);
        lost_data : out std_logic
    );
end monitor;

architecture mixed of monitor is

    -- component declaration
    component high_level_controller is
        port (
            clk, rst : in std_logic;
            system_clk : in std_logic;
            time_data_in : in std_logic_vector(63 downto 0);
            new_input : in std_logic;
			a_data_in : in std_logic;
			a_push_in : std_logic;
			b_data_in : in std_logic;
			b_push_in : std_logic;
			ID_data_in : in std_logic_vector(7 downto 0);
			ID_push_in : std_logic;
			a_data_out : out std_logic;
			a_en_out : out std_logic;
			b_data_out : out std_logic;
			b_en_out : out std_logic;
			ID_data_out : out signed(7 downto 0);
			ID_en_out : out std_logic;
			eq_en_out : out std_logic;
			lt_en_out : out std_logic;
			le_en_out : out std_logic;
			gt_en_out : out std_logic;
			ge_en_out : out std_logic;
			neq_en_out : out std_logic;
			not_a_en_out : out std_logic;
			a_and_b_en_out : out std_logic;
			a_or_b_en_out : out std_logic;
			a_impl_b_en_out : out std_logic;
			a_equiv_b_en_out : out std_logic;
			a_xor_b_en_out : out std_logic;
			true_const_en_out : out std_logic;
			time_stream_en_out : out std_logic;
            time_data_out : out unsigned(63 downto 0);
            push_data_in_query : out std_logic;
            lost_data : out std_logic
        );
    end component;

    component queue is
        port (
            clk, rst : in std_logic;
            push : in std_logic;
            time_data_in : in unsigned(63 downto 0);
			a_data_in : in std_logic;
			a_en_in : in std_logic;
			b_data_in : in std_logic;
			b_en_in : in std_logic;
			ID_data_in : in signed(7 downto 0);
			ID_en_in : in std_logic;
			eq_en_in : in std_logic;
			lt_en_in : in std_logic;
			le_en_in : in std_logic;
			gt_en_in : in std_logic;
			ge_en_in : in std_logic;
			neq_en_in : in std_logic;
			not_a_en_in : in std_logic;
			a_and_b_en_in : in std_logic;
			a_or_b_en_in : in std_logic;
			a_impl_b_en_in : in std_logic;
			a_equiv_b_en_in : in std_logic;
			a_xor_b_en_in : in std_logic;
			true_const_en_in : in std_logic;
			time_stream_en_in : in std_logic;
            full : out std_logic;
            pop : in std_logic;
            time_data_out : out unsigned(63 downto 0);
			a_data_out : out std_logic;
			a_en_out : out std_logic;
			b_data_out : out std_logic;
			b_en_out : out std_logic;
			ID_data_out : out signed(7 downto 0);
			ID_en_out : out std_logic;
			eq_en_out : out std_logic;
			lt_en_out : out std_logic;
			le_en_out : out std_logic;
			gt_en_out : out std_logic;
			ge_en_out : out std_logic;
			neq_en_out : out std_logic;
			not_a_en_out : out std_logic;
			a_and_b_en_out : out std_logic;
			a_or_b_en_out : out std_logic;
			a_impl_b_en_out : out std_logic;
			a_equiv_b_en_out : out std_logic;
			a_xor_b_en_out : out std_logic;
			true_const_en_out : out std_logic;
			time_stream_en_out : out std_logic;
            available : out std_logic
        );
    end component;

    component low_level_controller is
        port (
            clk, eclk, rst : in std_logic;
            time_in : in unsigned(63 downto 0);
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
            data_available : in std_logic;
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
		    pop : out std_logic;
            eval_done : out std_logic
        );
    end component;

    -- signal declarationq
    -- timing_manager
    signal push_data_timing : std_logic;
    signal timing_manager_time : unsigned(63 downto 0);
	signal a_data_timing : std_logic;
	signal a_en_timing : std_logic;
	signal b_data_timing : std_logic;
	signal b_en_timing : std_logic;
	signal ID_data_timing : signed(7 downto 0);
	signal ID_en_timing : std_logic;
	signal eq_en_timing : std_logic;
	signal lt_en_timing : std_logic;
	signal le_en_timing : std_logic;
	signal gt_en_timing : std_logic;
	signal ge_en_timing : std_logic;
	signal neq_en_timing : std_logic;
	signal not_a_en_timing : std_logic;
	signal a_and_b_en_timing : std_logic;
	signal a_or_b_en_timing : std_logic;
	signal a_impl_b_en_timing : std_logic;
	signal a_equiv_b_en_timing : std_logic;
	signal a_xor_b_en_timing : std_logic;
	signal true_const_en_timing : std_logic;
	signal time_stream_en_timing : std_logic;
    -- query
    signal queue_is_full : std_logic;
    signal queue_time : unsigned(63 downto 0);
    signal pop_queue : std_logic;
	signal a_data_queue : std_logic;
	signal a_en_queue : std_logic;
	signal b_data_queue : std_logic;
	signal b_en_queue : std_logic;
	signal ID_data_queue : signed(7 downto 0);
	signal ID_en_queue : std_logic;
	signal eq_en_queue : std_logic;
	signal lt_en_queue : std_logic;
	signal le_en_queue : std_logic;
	signal gt_en_queue : std_logic;
	signal ge_en_queue : std_logic;
	signal neq_en_queue : std_logic;
	signal not_a_en_queue : std_logic;
	signal a_and_b_en_queue : std_logic;
	signal a_or_b_en_queue : std_logic;
	signal a_impl_b_en_queue : std_logic;
	signal a_equiv_b_en_queue : std_logic;
	signal a_xor_b_en_queue : std_logic;
	signal true_const_en_queue : std_logic;
	signal time_stream_en_queue : std_logic;
    signal queue_data_available : std_logic;
    -- evaluator
	signal eq_stream_evaluator : std_logic;
	signal lt_stream_evaluator : std_logic;
	signal le_stream_evaluator : std_logic;
	signal gt_stream_evaluator : std_logic;
	signal ge_stream_evaluator : std_logic;
	signal neq_stream_evaluator : std_logic;
	signal not_a_stream_evaluator : std_logic;
	signal a_and_b_stream_evaluator : std_logic;
	signal a_or_b_stream_evaluator : std_logic;
	signal a_impl_b_stream_evaluator : std_logic;
	signal a_equiv_b_stream_evaluator : std_logic;
	signal a_xor_b_stream_evaluator : std_logic;
	signal true_const_stream_evaluator : std_logic;
	signal time_stream_stream_evaluator : signed(7 downto 0);
    -- monitor
    signal time_stream_reg : std_logic_vector(63 downto 0);
	signal a_stream_reg : std_logic;
	signal b_stream_reg : std_logic;
	signal ID_stream_reg : std_logic_vector(7 downto 0);
	signal eq_stream_reg : std_logic;
	signal lt_stream_reg : std_logic;
	signal le_stream_reg : std_logic;
	signal gt_stream_reg : std_logic;
	signal ge_stream_reg : std_logic;
	signal neq_stream_reg : std_logic;
	signal not_a_stream_reg : std_logic;
	signal a_and_b_stream_reg : std_logic;
	signal a_or_b_stream_reg : std_logic;
	signal a_impl_b_stream_reg : std_logic;
	signal a_equiv_b_stream_reg : std_logic;
	signal a_xor_b_stream_reg : std_logic;
	signal true_const_stream_reg : std_logic;
	signal time_stream_stream_reg : std_logic_vector(7 downto 0);

    signal print : std_logic;

begin
    -- component instantiation
    high_level_controller_instance: high_level_controller
        port map (
            clk => tclk,
            system_clk => clk,
            rst => rst,
            time_data_in => input_time,
            new_input => new_input,
			a_data_in => a_data_in,
			a_push_in => a_data_in_new_input,
			b_data_in => b_data_in,
			b_push_in => b_data_in_new_input,
			ID_data_in => ID_data_in,
			ID_push_in => ID_data_in_new_input,
			a_data_out => a_data_timing,
			a_en_out => a_en_timing,
			b_data_out => b_data_timing,
			b_en_out => b_en_timing,
			ID_data_out => ID_data_timing,
			ID_en_out => ID_en_timing,
			eq_en_out => eq_en_timing,
			lt_en_out => lt_en_timing,
			le_en_out => le_en_timing,
			gt_en_out => gt_en_timing,
			ge_en_out => ge_en_timing,
			neq_en_out => neq_en_timing,
			not_a_en_out => not_a_en_timing,
			a_and_b_en_out => a_and_b_en_timing,
			a_or_b_en_out => a_or_b_en_timing,
			a_impl_b_en_out => a_impl_b_en_timing,
			a_equiv_b_en_out => a_equiv_b_en_timing,
			a_xor_b_en_out => a_xor_b_en_timing,
			true_const_en_out => true_const_en_timing,
			time_stream_en_out => time_stream_en_timing,
            time_data_out => timing_manager_time,
            push_data_in_query => push_data_timing,
            lost_data => lost_data
        );

    queue_instance: queue
        port map (
            clk => qclk,
            rst => rst,
            push => push_data_timing,
            time_data_in => timing_manager_time,
			a_data_in => a_data_timing,
			a_en_in => a_en_timing,
			b_data_in => b_data_timing,
			b_en_in => b_en_timing,
			ID_data_in => ID_data_timing,
			ID_en_in => ID_en_timing,
			eq_en_in => eq_en_timing,
			lt_en_in => lt_en_timing,
			le_en_in => le_en_timing,
			gt_en_in => gt_en_timing,
			ge_en_in => ge_en_timing,
			neq_en_in => neq_en_timing,
			not_a_en_in => not_a_en_timing,
			a_and_b_en_in => a_and_b_en_timing,
			a_or_b_en_in => a_or_b_en_timing,
			a_impl_b_en_in => a_impl_b_en_timing,
			a_equiv_b_en_in => a_equiv_b_en_timing,
			a_xor_b_en_in => a_xor_b_en_timing,
			true_const_en_in => true_const_en_timing,
			time_stream_en_in => time_stream_en_timing,
            full => queue_is_full,
            pop => pop_queue,
            time_data_out => queue_time,
			a_data_out => a_data_queue,
			a_en_out => a_en_queue,
			b_data_out => b_data_queue,
			b_en_out => b_en_queue,
			ID_data_out => ID_data_queue,
			ID_en_out => ID_en_queue,
			eq_en_out => eq_en_queue,
			lt_en_out => lt_en_queue,
			le_en_out => le_en_queue,
			gt_en_out => gt_en_queue,
			ge_en_out => ge_en_queue,
			neq_en_out => neq_en_queue,
			not_a_en_out => not_a_en_queue,
			a_and_b_en_out => a_and_b_en_queue,
			a_or_b_en_out => a_or_b_en_queue,
			a_impl_b_en_out => a_impl_b_en_queue,
			a_equiv_b_en_out => a_equiv_b_en_queue,
			a_xor_b_en_out => a_xor_b_en_queue,
			true_const_en_out => true_const_en_queue,
			time_stream_en_out => time_stream_en_queue,
            available => queue_data_available
        );

    low_level_controller_instance: low_level_controller
        port map (
            clk => clk,
            eclk => eclk,
            rst => rst,
            time_in => queue_time,
			a => a_data_queue,
			a_en => a_en_queue,
			b => b_data_queue,
			b_en => b_en_queue,
			ID => ID_data_queue,
			ID_en => ID_en_queue,
			eq_en => eq_en_queue,
			lt_en => lt_en_queue,
			le_en => le_en_queue,
			gt_en => gt_en_queue,
			ge_en => ge_en_queue,
			neq_en => neq_en_queue,
			not_a_en => not_a_en_queue,
			a_and_b_en => a_and_b_en_queue,
			a_or_b_en => a_or_b_en_queue,
			a_impl_b_en => a_impl_b_en_queue,
			a_equiv_b_en => a_equiv_b_en_queue,
			a_xor_b_en => a_xor_b_en_queue,
			true_const_en => true_const_en_queue,
			time_stream_en => time_stream_en_queue,
            data_available => queue_data_available,
			eq => eq_stream_evaluator,
			lt => lt_stream_evaluator,
			le => le_stream_evaluator,
			gt => gt_stream_evaluator,
			ge => ge_stream_evaluator,
			neq => neq_stream_evaluator,
			not_a => not_a_stream_evaluator,
			a_and_b => a_and_b_stream_evaluator,
			a_or_b => a_or_b_stream_evaluator,
			a_impl_b => a_impl_b_stream_evaluator,
			a_equiv_b => a_equiv_b_stream_evaluator,
			a_xor_b => a_xor_b_stream_evaluator,
			true_const => true_const_stream_evaluator,
			time_stream => time_stream_stream_evaluator,
            pop => pop_queue,
            eval_done => print
        );

    process(rst, print) begin
        if (rst = '1') then
            time_stream_reg <= (others => '0');
			a_stream_reg <= '0';
			b_stream_reg <= '0';
			ID_stream_reg <= (others => '0');
			eq_stream_reg <= '0';
			lt_stream_reg <= '0';
			le_stream_reg <= '0';
			gt_stream_reg <= '0';
			ge_stream_reg <= '0';
			neq_stream_reg <= '0';
			not_a_stream_reg <= '0';
			a_and_b_stream_reg <= '0';
			a_or_b_stream_reg <= '0';
			a_impl_b_stream_reg <= '0';
			a_equiv_b_stream_reg <= '0';
			a_xor_b_stream_reg <= '0';
			true_const_stream_reg <= '0';
			time_stream_stream_reg <= (others => '0');
        elsif falling_edge(print) then
            time_stream_reg <= std_logic_vector(queue_time);
			a_stream_reg <= a_data_queue;
			b_stream_reg <= b_data_queue;
			ID_stream_reg <= std_logic_vector(ID_data_queue);
			eq_stream_reg <= eq_stream_evaluator;
			lt_stream_reg <= lt_stream_evaluator;
			le_stream_reg <= le_stream_evaluator;
			gt_stream_reg <= gt_stream_evaluator;
			ge_stream_reg <= ge_stream_evaluator;
			neq_stream_reg <= neq_stream_evaluator;
			not_a_stream_reg <= not_a_stream_evaluator;
			a_and_b_stream_reg <= a_and_b_stream_evaluator;
			a_or_b_stream_reg <= a_or_b_stream_evaluator;
			a_impl_b_stream_reg <= a_impl_b_stream_evaluator;
			a_equiv_b_stream_reg <= a_equiv_b_stream_evaluator;
			a_xor_b_stream_reg <= a_xor_b_stream_evaluator;
			true_const_stream_reg <= true_const_stream_evaluator;
			time_stream_stream_reg <= std_logic_vector(time_stream_stream_evaluator);
        end if;
    end process;

    time_stream <= time_stream_reg;
	a_stream <= a_stream_reg;
	b_stream <= b_stream_reg;
	ID_stream <= ID_stream_reg;
	eq_stream <= eq_stream_reg;
	lt_stream <= lt_stream_reg;
	le_stream <= le_stream_reg;
	gt_stream <= gt_stream_reg;
	ge_stream <= ge_stream_reg;
	neq_stream <= neq_stream_reg;
	not_a_stream <= not_a_stream_reg;
	a_and_b_stream <= a_and_b_stream_reg;
	a_or_b_stream <= a_or_b_stream_reg;
	a_impl_b_stream <= a_impl_b_stream_reg;
	a_equiv_b_stream <= a_equiv_b_stream_reg;
	a_xor_b_stream <= a_xor_b_stream_reg;
	true_const_stream <= true_const_stream_reg;
	time_stream_stream <= time_stream_stream_reg;

end mixed;
