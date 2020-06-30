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
		s_s_stream: out std_logic_vector(31 downto 0);
		c_s_stream: out std_logic_vector(63 downto 0);
		av_s_stream: out std_logic_vector(31 downto 0);
		a_u_stream: out std_logic_vector(31 downto 0);
		s_u_stream: out std_logic_vector(31 downto 0);
		c_u_stream: out std_logic_vector(63 downto 0);
		av_u_stream: out std_logic_vector(31 downto 0);
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
			s_s_en_out : out std_logic;
			c_s_en_out : out std_logic;
			av_s_en_out : out std_logic;
			a_u_en_out : out std_logic;
			s_u_en_out : out std_logic;
			c_u_en_out : out std_logic;
			av_u_en_out : out std_logic;
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
			s_s_en_in : in std_logic;
			c_s_en_in : in std_logic;
			av_s_en_in : in std_logic;
			a_u_en_in : in std_logic;
			s_u_en_in : in std_logic;
			c_u_en_in : in std_logic;
			av_u_en_in : in std_logic;
            full : out std_logic;
            pop : in std_logic;
            time_data_out : out unsigned(63 downto 0);
			a_data_out : out signed(31 downto 0);
			a_en_out : out std_logic;
			s_s_en_out : out std_logic;
			c_s_en_out : out std_logic;
			av_s_en_out : out std_logic;
			a_u_en_out : out std_logic;
			s_u_en_out : out std_logic;
			c_u_en_out : out std_logic;
			av_u_en_out : out std_logic;
            available : out std_logic
        );
    end component;

    component low_level_controller is
        port (
            clk, eclk, rst : in std_logic;
            time_in : in unsigned(63 downto 0);
			a : in signed(31 downto 0);
			a_en : in std_logic;
			s_s_en : in std_logic;
			c_s_en : in std_logic;
			av_s_en : in std_logic;
			a_u_en : in std_logic;
			s_u_en : in std_logic;
			c_u_en : in std_logic;
			av_u_en : in std_logic;
            data_available : in std_logic;
			s_s : out signed(31 downto 0);
			c_s : out unsigned(63 downto 0);
			av_s : out signed(31 downto 0);
			a_u : out unsigned(31 downto 0);
			s_u : out unsigned(31 downto 0);
			c_u : out unsigned(63 downto 0);
			av_u : out unsigned(31 downto 0);
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
	signal s_s_en_timing : std_logic;
	signal c_s_en_timing : std_logic;
	signal av_s_en_timing : std_logic;
	signal a_u_en_timing : std_logic;
	signal s_u_en_timing : std_logic;
	signal c_u_en_timing : std_logic;
	signal av_u_en_timing : std_logic;
    -- query
    signal queue_is_full : std_logic;
    signal queue_time : unsigned(63 downto 0);
    signal pop_queue : std_logic;
	signal a_data_queue : signed(31 downto 0);
	signal a_en_queue : std_logic;
	signal s_s_en_queue : std_logic;
	signal c_s_en_queue : std_logic;
	signal av_s_en_queue : std_logic;
	signal a_u_en_queue : std_logic;
	signal s_u_en_queue : std_logic;
	signal c_u_en_queue : std_logic;
	signal av_u_en_queue : std_logic;
    signal queue_data_available : std_logic;
    -- evaluator
	signal s_s_stream_evaluator : signed(31 downto 0);
	signal c_s_stream_evaluator : unsigned(63 downto 0);
	signal av_s_stream_evaluator : signed(31 downto 0);
	signal a_u_stream_evaluator : unsigned(31 downto 0);
	signal s_u_stream_evaluator : unsigned(31 downto 0);
	signal c_u_stream_evaluator : unsigned(63 downto 0);
	signal av_u_stream_evaluator : unsigned(31 downto 0);
    -- monitor
    signal time_stream_reg : std_logic_vector(63 downto 0);
	signal a_stream_reg : std_logic_vector(31 downto 0);
	signal s_s_stream_reg : std_logic_vector(31 downto 0);
	signal c_s_stream_reg : std_logic_vector(63 downto 0);
	signal av_s_stream_reg : std_logic_vector(31 downto 0);
	signal a_u_stream_reg : std_logic_vector(31 downto 0);
	signal s_u_stream_reg : std_logic_vector(31 downto 0);
	signal c_u_stream_reg : std_logic_vector(63 downto 0);
	signal av_u_stream_reg : std_logic_vector(31 downto 0);

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
			s_s_en_out => s_s_en_timing,
			c_s_en_out => c_s_en_timing,
			av_s_en_out => av_s_en_timing,
			a_u_en_out => a_u_en_timing,
			s_u_en_out => s_u_en_timing,
			c_u_en_out => c_u_en_timing,
			av_u_en_out => av_u_en_timing,
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
			s_s_en_in => s_s_en_timing,
			c_s_en_in => c_s_en_timing,
			av_s_en_in => av_s_en_timing,
			a_u_en_in => a_u_en_timing,
			s_u_en_in => s_u_en_timing,
			c_u_en_in => c_u_en_timing,
			av_u_en_in => av_u_en_timing,
            full => queue_is_full,
            pop => pop_queue,
            time_data_out => queue_time,
			a_data_out => a_data_queue,
			a_en_out => a_en_queue,
			s_s_en_out => s_s_en_queue,
			c_s_en_out => c_s_en_queue,
			av_s_en_out => av_s_en_queue,
			a_u_en_out => a_u_en_queue,
			s_u_en_out => s_u_en_queue,
			c_u_en_out => c_u_en_queue,
			av_u_en_out => av_u_en_queue,
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
			s_s_en => s_s_en_queue,
			c_s_en => c_s_en_queue,
			av_s_en => av_s_en_queue,
			a_u_en => a_u_en_queue,
			s_u_en => s_u_en_queue,
			c_u_en => c_u_en_queue,
			av_u_en => av_u_en_queue,
            data_available => queue_data_available,
			s_s => s_s_stream_evaluator,
			c_s => c_s_stream_evaluator,
			av_s => av_s_stream_evaluator,
			a_u => a_u_stream_evaluator,
			s_u => s_u_stream_evaluator,
			c_u => c_u_stream_evaluator,
			av_u => av_u_stream_evaluator,
            pop => pop_queue,
            eval_done => print
        );

    process(rst, print) begin
        if (rst = '1') then
            time_stream_reg <= (others => '0');
			a_stream_reg <= (others => '0');
			s_s_stream_reg <= (others => '0');
			c_s_stream_reg <= (others => '0');
			av_s_stream_reg <= (others => '0');
			a_u_stream_reg <= (others => '0');
			s_u_stream_reg <= (others => '0');
			c_u_stream_reg <= (others => '0');
			av_u_stream_reg <= (others => '0');
        elsif falling_edge(print) then
            time_stream_reg <= std_logic_vector(queue_time);
			a_stream_reg <= std_logic_vector(a_data_queue);
			s_s_stream_reg <= std_logic_vector(s_s_stream_evaluator);
			c_s_stream_reg <= std_logic_vector(c_s_stream_evaluator);
			av_s_stream_reg <= std_logic_vector(av_s_stream_evaluator);
			a_u_stream_reg <= std_logic_vector(a_u_stream_evaluator);
			s_u_stream_reg <= std_logic_vector(s_u_stream_evaluator);
			c_u_stream_reg <= std_logic_vector(c_u_stream_evaluator);
			av_u_stream_reg <= std_logic_vector(av_u_stream_evaluator);
        end if;
    end process;

    time_stream <= time_stream_reg;
	a_stream <= a_stream_reg;
	s_s_stream <= s_s_stream_reg;
	c_s_stream <= c_s_stream_reg;
	av_s_stream <= av_s_stream_reg;
	a_u_stream <= a_u_stream_reg;
	s_u_stream <= s_u_stream_reg;
	c_u_stream <= c_u_stream_reg;
	av_u_stream <= av_u_stream_reg;

end mixed;
