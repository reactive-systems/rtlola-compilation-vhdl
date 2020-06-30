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
		a_data_in : in std_logic_vector(31 downto 0);
		a_data_in_new_input : in std_logic;
		b_data_in : in std_logic_vector(31 downto 0);
		b_data_in_new_input : in std_logic;
        time_stream : out std_logic_vector(63 downto 0);
		a_stream: out std_logic_vector(31 downto 0);
		b_stream: out std_logic_vector(31 downto 0);
		plus_op_stream: out std_logic_vector(31 downto 0);
		minus_op_stream: out std_logic_vector(31 downto 0);
		mult_op_stream: out std_logic_vector(31 downto 0);
		div_op_stream: out std_logic_vector(31 downto 0);
		mod_op_stream: out std_logic_vector(31 downto 0);
		func_abs_stream: out std_logic_vector(31 downto 0);
		func_sqrt_stream: out std_logic_vector(31 downto 0);
		counter_stream: out std_logic_vector(31 downto 0);
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
			a_data_in : in std_logic_vector(31 downto 0);
			a_push_in : std_logic;
			b_data_in : in std_logic_vector(31 downto 0);
			b_push_in : std_logic;
			a_data_out : out signed(31 downto 0);
			a_en_out : out std_logic;
			b_data_out : out signed(31 downto 0);
			b_en_out : out std_logic;
			plus_op_en_out : out std_logic;
			minus_op_en_out : out std_logic;
			mult_op_en_out : out std_logic;
			div_op_en_out : out std_logic;
			mod_op_en_out : out std_logic;
			func_abs_en_out : out std_logic;
			func_sqrt_en_out : out std_logic;
			counter_en_out : out std_logic;
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
			a_data_in : in signed(31 downto 0);
			a_en_in : in std_logic;
			b_data_in : in signed(31 downto 0);
			b_en_in : in std_logic;
			plus_op_en_in : in std_logic;
			minus_op_en_in : in std_logic;
			mult_op_en_in : in std_logic;
			div_op_en_in : in std_logic;
			mod_op_en_in : in std_logic;
			func_abs_en_in : in std_logic;
			func_sqrt_en_in : in std_logic;
			counter_en_in : in std_logic;
            full : out std_logic;
            pop : in std_logic;
            time_data_out : out unsigned(63 downto 0);
			a_data_out : out signed(31 downto 0);
			a_en_out : out std_logic;
			b_data_out : out signed(31 downto 0);
			b_en_out : out std_logic;
			plus_op_en_out : out std_logic;
			minus_op_en_out : out std_logic;
			mult_op_en_out : out std_logic;
			div_op_en_out : out std_logic;
			mod_op_en_out : out std_logic;
			func_abs_en_out : out std_logic;
			func_sqrt_en_out : out std_logic;
			counter_en_out : out std_logic;
            available : out std_logic
        );
    end component;

    component low_level_controller is
        port (
            clk, eclk, rst : in std_logic;
            time_in : in unsigned(63 downto 0);
			a : in signed(31 downto 0);
			a_en : in std_logic;
			b : in signed(31 downto 0);
			b_en : in std_logic;
			plus_op_en : in std_logic;
			minus_op_en : in std_logic;
			mult_op_en : in std_logic;
			div_op_en : in std_logic;
			mod_op_en : in std_logic;
			func_abs_en : in std_logic;
			func_sqrt_en : in std_logic;
			counter_en : in std_logic;
            data_available : in std_logic;
			plus_op : out signed(31 downto 0);
			minus_op : out signed(31 downto 0);
			mult_op : out signed(31 downto 0);
			div_op : out signed(31 downto 0);
			mod_op : out signed(31 downto 0);
			func_abs : out signed(31 downto 0);
			func_sqrt : out signed(31 downto 0);
			counter : out signed(31 downto 0);
		    pop : out std_logic;
            eval_done : out std_logic
        );
    end component;

    -- signal declarationq
    -- timing_manager
    signal push_data_timing : std_logic;
    signal timing_manager_time : unsigned(63 downto 0);
	signal a_data_timing : signed(31 downto 0);
	signal a_en_timing : std_logic;
	signal b_data_timing : signed(31 downto 0);
	signal b_en_timing : std_logic;
	signal plus_op_en_timing : std_logic;
	signal minus_op_en_timing : std_logic;
	signal mult_op_en_timing : std_logic;
	signal div_op_en_timing : std_logic;
	signal mod_op_en_timing : std_logic;
	signal func_abs_en_timing : std_logic;
	signal func_sqrt_en_timing : std_logic;
	signal counter_en_timing : std_logic;
    -- query
    signal queue_is_full : std_logic;
    signal queue_time : unsigned(63 downto 0);
    signal pop_queue : std_logic;
	signal a_data_queue : signed(31 downto 0);
	signal a_en_queue : std_logic;
	signal b_data_queue : signed(31 downto 0);
	signal b_en_queue : std_logic;
	signal plus_op_en_queue : std_logic;
	signal minus_op_en_queue : std_logic;
	signal mult_op_en_queue : std_logic;
	signal div_op_en_queue : std_logic;
	signal mod_op_en_queue : std_logic;
	signal func_abs_en_queue : std_logic;
	signal func_sqrt_en_queue : std_logic;
	signal counter_en_queue : std_logic;
    signal queue_data_available : std_logic;
    -- evaluator
	signal plus_op_stream_evaluator : signed(31 downto 0);
	signal minus_op_stream_evaluator : signed(31 downto 0);
	signal mult_op_stream_evaluator : signed(31 downto 0);
	signal div_op_stream_evaluator : signed(31 downto 0);
	signal mod_op_stream_evaluator : signed(31 downto 0);
	signal func_abs_stream_evaluator : signed(31 downto 0);
	signal func_sqrt_stream_evaluator : signed(31 downto 0);
	signal counter_stream_evaluator : signed(31 downto 0);
    -- monitor
    signal time_stream_reg : std_logic_vector(63 downto 0);
	signal a_stream_reg : std_logic_vector(31 downto 0);
	signal b_stream_reg : std_logic_vector(31 downto 0);
	signal plus_op_stream_reg : std_logic_vector(31 downto 0);
	signal minus_op_stream_reg : std_logic_vector(31 downto 0);
	signal mult_op_stream_reg : std_logic_vector(31 downto 0);
	signal div_op_stream_reg : std_logic_vector(31 downto 0);
	signal mod_op_stream_reg : std_logic_vector(31 downto 0);
	signal func_abs_stream_reg : std_logic_vector(31 downto 0);
	signal func_sqrt_stream_reg : std_logic_vector(31 downto 0);
	signal counter_stream_reg : std_logic_vector(31 downto 0);

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
			a_data_out => a_data_timing,
			a_en_out => a_en_timing,
			b_data_out => b_data_timing,
			b_en_out => b_en_timing,
			plus_op_en_out => plus_op_en_timing,
			minus_op_en_out => minus_op_en_timing,
			mult_op_en_out => mult_op_en_timing,
			div_op_en_out => div_op_en_timing,
			mod_op_en_out => mod_op_en_timing,
			func_abs_en_out => func_abs_en_timing,
			func_sqrt_en_out => func_sqrt_en_timing,
			counter_en_out => counter_en_timing,
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
			plus_op_en_in => plus_op_en_timing,
			minus_op_en_in => minus_op_en_timing,
			mult_op_en_in => mult_op_en_timing,
			div_op_en_in => div_op_en_timing,
			mod_op_en_in => mod_op_en_timing,
			func_abs_en_in => func_abs_en_timing,
			func_sqrt_en_in => func_sqrt_en_timing,
			counter_en_in => counter_en_timing,
            full => queue_is_full,
            pop => pop_queue,
            time_data_out => queue_time,
			a_data_out => a_data_queue,
			a_en_out => a_en_queue,
			b_data_out => b_data_queue,
			b_en_out => b_en_queue,
			plus_op_en_out => plus_op_en_queue,
			minus_op_en_out => minus_op_en_queue,
			mult_op_en_out => mult_op_en_queue,
			div_op_en_out => div_op_en_queue,
			mod_op_en_out => mod_op_en_queue,
			func_abs_en_out => func_abs_en_queue,
			func_sqrt_en_out => func_sqrt_en_queue,
			counter_en_out => counter_en_queue,
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
			plus_op_en => plus_op_en_queue,
			minus_op_en => minus_op_en_queue,
			mult_op_en => mult_op_en_queue,
			div_op_en => div_op_en_queue,
			mod_op_en => mod_op_en_queue,
			func_abs_en => func_abs_en_queue,
			func_sqrt_en => func_sqrt_en_queue,
			counter_en => counter_en_queue,
            data_available => queue_data_available,
			plus_op => plus_op_stream_evaluator,
			minus_op => minus_op_stream_evaluator,
			mult_op => mult_op_stream_evaluator,
			div_op => div_op_stream_evaluator,
			mod_op => mod_op_stream_evaluator,
			func_abs => func_abs_stream_evaluator,
			func_sqrt => func_sqrt_stream_evaluator,
			counter => counter_stream_evaluator,
            pop => pop_queue,
            eval_done => print
        );

    process(rst, print) begin
        if (rst = '1') then
            time_stream_reg <= (others => '0');
			a_stream_reg <= (others => '0');
			b_stream_reg <= (others => '0');
			plus_op_stream_reg <= (others => '0');
			minus_op_stream_reg <= (others => '0');
			mult_op_stream_reg <= (others => '0');
			div_op_stream_reg <= (others => '0');
			mod_op_stream_reg <= (others => '0');
			func_abs_stream_reg <= (others => '0');
			func_sqrt_stream_reg <= (others => '0');
			counter_stream_reg <= (others => '0');
        elsif falling_edge(print) then
            time_stream_reg <= std_logic_vector(queue_time);
			a_stream_reg <= std_logic_vector(a_data_queue);
			b_stream_reg <= std_logic_vector(b_data_queue);
			plus_op_stream_reg <= std_logic_vector(plus_op_stream_evaluator);
			minus_op_stream_reg <= std_logic_vector(minus_op_stream_evaluator);
			mult_op_stream_reg <= std_logic_vector(mult_op_stream_evaluator);
			div_op_stream_reg <= std_logic_vector(div_op_stream_evaluator);
			mod_op_stream_reg <= std_logic_vector(mod_op_stream_evaluator);
			func_abs_stream_reg <= std_logic_vector(func_abs_stream_evaluator);
			func_sqrt_stream_reg <= std_logic_vector(func_sqrt_stream_evaluator);
			counter_stream_reg <= std_logic_vector(counter_stream_evaluator);
        end if;
    end process;

    time_stream <= time_stream_reg;
	a_stream <= a_stream_reg;
	b_stream <= b_stream_reg;
	plus_op_stream <= plus_op_stream_reg;
	minus_op_stream <= minus_op_stream_reg;
	mult_op_stream <= mult_op_stream_reg;
	div_op_stream <= div_op_stream_reg;
	mod_op_stream <= mod_op_stream_reg;
	func_abs_stream <= func_abs_stream_reg;
	func_sqrt_stream <= func_sqrt_stream_reg;
	counter_stream <= counter_stream_reg;

end mixed;
