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
        time_stream : out std_logic_vector(63 downto 0);
		a_stream: out std_logic_vector(31 downto 0);
		b_stream: out std_logic_vector(31 downto 0);
		c_stream: out std_logic_vector(31 downto 0);
		d_stream: out std_logic_vector(31 downto 0);
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
			a_data_out : out signed(31 downto 0);
			a_en_out : out std_logic;
			b_en_out : out std_logic;
			c_en_out : out std_logic;
			d_en_out : out std_logic;
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
			b_en_in : in std_logic;
			c_en_in : in std_logic;
			d_en_in : in std_logic;
            full : out std_logic;
            pop : in std_logic;
            time_data_out : out unsigned(63 downto 0);
			a_data_out : out signed(31 downto 0);
			a_en_out : out std_logic;
			b_en_out : out std_logic;
			c_en_out : out std_logic;
			d_en_out : out std_logic;
            available : out std_logic
        );
    end component;

    component low_level_controller is
        port (
            clk, eclk, rst : in std_logic;
            time_in : in unsigned(63 downto 0);
			a : in signed(31 downto 0);
			a_en : in std_logic;
			b_en : in std_logic;
			c_en : in std_logic;
			d_en : in std_logic;
            data_available : in std_logic;
			b : out signed(31 downto 0);
			c : out signed(31 downto 0);
			d : out signed(31 downto 0);
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
	signal b_en_timing : std_logic;
	signal c_en_timing : std_logic;
	signal d_en_timing : std_logic;
    -- query
    signal queue_is_full : std_logic;
    signal queue_time : unsigned(63 downto 0);
    signal pop_queue : std_logic;
	signal a_data_queue : signed(31 downto 0);
	signal a_en_queue : std_logic;
	signal b_en_queue : std_logic;
	signal c_en_queue : std_logic;
	signal d_en_queue : std_logic;
    signal queue_data_available : std_logic;
    -- evaluator
	signal b_stream_evaluator : signed(31 downto 0);
	signal c_stream_evaluator : signed(31 downto 0);
	signal d_stream_evaluator : signed(31 downto 0);
    -- monitor
    signal time_stream_reg : std_logic_vector(63 downto 0);
	signal a_stream_reg : std_logic_vector(31 downto 0);
	signal b_stream_reg : std_logic_vector(31 downto 0);
	signal c_stream_reg : std_logic_vector(31 downto 0);
	signal d_stream_reg : std_logic_vector(31 downto 0);

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
			a_data_out => a_data_timing,
			a_en_out => a_en_timing,
			b_en_out => b_en_timing,
			c_en_out => c_en_timing,
			d_en_out => d_en_timing,
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
			b_en_in => b_en_timing,
			c_en_in => c_en_timing,
			d_en_in => d_en_timing,
            full => queue_is_full,
            pop => pop_queue,
            time_data_out => queue_time,
			a_data_out => a_data_queue,
			a_en_out => a_en_queue,
			b_en_out => b_en_queue,
			c_en_out => c_en_queue,
			d_en_out => d_en_queue,
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
			b_en => b_en_queue,
			c_en => c_en_queue,
			d_en => d_en_queue,
            data_available => queue_data_available,
			b => b_stream_evaluator,
			c => c_stream_evaluator,
			d => d_stream_evaluator,
            pop => pop_queue,
            eval_done => print
        );

    process(rst, print) begin
        if (rst = '1') then
            time_stream_reg <= (others => '0');
			a_stream_reg <= (others => '0');
			b_stream_reg <= (others => '0');
			c_stream_reg <= (others => '0');
			d_stream_reg <= (others => '0');
        elsif falling_edge(print) then
            time_stream_reg <= std_logic_vector(queue_time);
			a_stream_reg <= std_logic_vector(a_data_queue);
			b_stream_reg <= std_logic_vector(b_stream_evaluator);
			c_stream_reg <= std_logic_vector(c_stream_evaluator);
			d_stream_reg <= std_logic_vector(d_stream_evaluator);
        end if;
    end process;

    time_stream <= time_stream_reg;
	a_stream <= a_stream_reg;
	b_stream <= b_stream_reg;
	c_stream <= c_stream_reg;
	d_stream <= d_stream_reg;

end mixed;
