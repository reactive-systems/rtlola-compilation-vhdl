library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

entity high_level_controller is
    port(
        clk, rst : in std_logic;
        system_clk : in std_logic;
        time_data_in : in std_logic_vector(63 downto 0);
        new_input : in std_logic;
		a_data_in : in std_logic_vector(31 downto 0);
		a_push_in : in std_logic;
		b_data_in : in std_logic_vector(31 downto 0);
		b_push_in : in std_logic;
		c_data_in : in std_logic_vector(31 downto 0);
		c_push_in : in std_logic;
		a_data_out : out sfixed(8 downto -23);
		a_en_out : out std_logic;
		b_data_out : out sfixed(8 downto -23);
		b_en_out : out std_logic;
		c_data_out : out sfixed(8 downto -23);
		c_en_out : out std_logic;
		plus_op_en_out : out std_logic;
		minus_op_en_out : out std_logic;
		mult_op_en_out : out std_logic;
		func_abs_en_out : out std_logic;
		func_sqrt_en_out : out std_logic;
		counter_en_out : out std_logic;
        time_data_out : out unsigned(63 downto 0);
        push_data_in_query : out std_logic;
        lost_data : out std_logic
    );
end high_level_controller;

architecture mixed of high_level_controller is

component extInterface
    port (
        clk, rst : in std_logic;
        time_in : in std_logic_vector(63 downto 0);
		a_data_in : in std_logic_vector(31 downto 0);
		a_push_in : in std_logic;
		b_data_in : in std_logic_vector(31 downto 0);
		b_push_in : in std_logic;
		c_data_in : in std_logic_vector(31 downto 0);
		c_push_in : in std_logic;
		a_data_out : out sfixed(8 downto -23);
		a_push_out : out std_logic;
		b_data_out : out sfixed(8 downto -23);
		b_push_out : out std_logic;
		c_data_out : out sfixed(8 downto -23);
		c_push_out : out std_logic;
        time_out : out unsigned(63 downto 0)
    );
end component;


component check_new_input
    port (
        clk, rst : in std_logic;
        new_input_in : in std_logic;
        new_input_out : out std_logic
    );
end component;


component event_delay
    port (
        clk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
		a_data_in : in sfixed(8 downto -23);
		a_push_in : in std_logic;
		b_data_in : in sfixed(8 downto -23);
		b_push_in : in std_logic;
		c_data_in : in sfixed(8 downto -23);
		c_push_in : in std_logic;
        push_event_in : in std_logic;
        time_out : out unsigned(63 downto 0);
		a_data_out : out sfixed(8 downto -23);
		a_push_out : out std_logic;
		b_data_out : out sfixed(8 downto -23);
		b_push_out : out std_logic;
		c_data_out : out sfixed(8 downto -23);
		c_push_out : out std_logic;
        push_event_out : out std_logic
    );
end component;

component event_scheduler
    port (
        clk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
        hold : in std_logic;
		a_data_in : in sfixed(8 downto -23);
		a_push_in : in std_logic;
		b_data_in : in sfixed(8 downto -23);
		b_push_in : in std_logic;
		c_data_in : in sfixed(8 downto -23);
		c_push_in : in std_logic;
        push_event_in : in std_logic;
        time_out : out unsigned(63 downto 0);
        push_event_out : out std_logic;
		a_data_out : out sfixed(8 downto -23);
		a_push_out : out std_logic;
		b_data_out : out sfixed(8 downto -23);
		b_push_out : out std_logic;
		c_data_out : out sfixed(8 downto -23);
		c_push_out : out std_logic;
        lost_data : out std_logic
    );
end component;


component time_unit
    port(
        clk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
        push : in std_logic;
        time_out : out unsigned(63 downto 0)
    );
end component;


component scheduler
    port (
        clk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
        time_last_deadline_out : out unsigned(63 downto 0);
        hit_deadline_out : out std_logic
    );
end component;


component hlQInterface is
    port (
        clk, rst : in std_logic;
        time_for_event : in unsigned(63 downto 0);
        time_for_deadline : in unsigned(63 downto 0);
        push_event : in std_logic;
        push_deadline : in std_logic;
		a_data_in : in sfixed(8 downto -23);
		a_en_in : in std_logic;
		b_data_in : in sfixed(8 downto -23);
		b_en_in : in std_logic;
		c_data_in : in sfixed(8 downto -23);
		c_en_in : in std_logic;
		a_data_out : out sfixed(8 downto -23);
		a_en_out : out std_logic;
		b_data_out : out sfixed(8 downto -23);
		b_en_out : out std_logic;
		c_data_out : out sfixed(8 downto -23);
		c_en_out : out std_logic;
		plus_op_en_out : out std_logic;
		minus_op_en_out : out std_logic;
		mult_op_en_out : out std_logic;
		func_abs_en_out : out std_logic;
		func_sqrt_en_out : out std_logic;
		counter_en_out : out std_logic;
        time_out : out unsigned(63 downto 0);
        push_out : out std_logic
    );
end component;

    -- Internal Signal Declarations
    signal time_from_extInterface : unsigned(63 downto 0);
    signal time_from_time_unit : unsigned(63 downto 0);
    signal time_from_eventDelay : unsigned(63 downto 0);
    signal time_from_eventScheduler : unsigned(63 downto 0);
    signal time_from_scheduler : unsigned(63 downto 0);
    signal push_from_new_input_check : std_logic;
    signal push_from_eventDelay : std_logic;
    signal push_from_eventScheduler : std_logic;
    
	signal a_data_from_extInterface : sfixed(8 downto -23);
	signal a_push_from_extInterface : std_logic;
	signal a_data_from_eventDelay : sfixed(8 downto -23);
	signal a_push_from_eventDelay : std_logic;
	signal a_data_from_eventScheduler : sfixed(8 downto -23);
	signal a_push_from_eventScheduler : std_logic;
	signal b_data_from_extInterface : sfixed(8 downto -23);
	signal b_push_from_extInterface : std_logic;
	signal b_data_from_eventDelay : sfixed(8 downto -23);
	signal b_push_from_eventDelay : std_logic;
	signal b_data_from_eventScheduler : sfixed(8 downto -23);
	signal b_push_from_eventScheduler : std_logic;
	signal c_data_from_extInterface : sfixed(8 downto -23);
	signal c_push_from_extInterface : std_logic;
	signal c_data_from_eventDelay : sfixed(8 downto -23);
	signal c_push_from_eventDelay : std_logic;
	signal c_data_from_eventScheduler : sfixed(8 downto -23);
	signal c_push_from_eventScheduler : std_logic;

    signal hit_deadline : std_logic;
    signal hlc_clk_count : integer;
    signal slow_hlc_clk : std_logic;

begin
    extInterface_instance: extInterface
        port map (
            clk => slow_hlc_clk,
            rst => rst,
            time_in => time_data_in,
			a_data_in => a_data_in,
			a_push_in => a_push_in,
			b_data_in => b_data_in,
			b_push_in => b_push_in,
			c_data_in => c_data_in,
			c_push_in => c_push_in,
			a_data_out => a_data_from_extInterface,
			a_push_out => a_push_from_extInterface,
			b_data_out => b_data_from_extInterface,
			b_push_out => b_push_from_extInterface,
			c_data_out => c_data_from_extInterface,
			c_push_out => c_push_from_extInterface,
            time_out => time_from_extInterface
        );

    check_new_input_instance: check_new_input
        port map (
            clk => slow_hlc_clk,
            rst => rst,
            new_input_in => new_input,
            new_input_out => push_from_new_input_check
        );

    event_delay_instance: event_delay
        port map(
            clk => slow_hlc_clk,
            rst => rst,
            time_in => time_from_time_unit,
			a_data_in => a_data_from_extInterface,
			a_push_in => a_push_from_extInterface,
			b_data_in => b_data_from_extInterface,
			b_push_in => b_push_from_extInterface,
			c_data_in => c_data_from_extInterface,
			c_push_in => c_push_from_extInterface,
            push_event_in => push_from_new_input_check,
			a_data_out => a_data_from_eventDelay,
			a_push_out => a_push_from_eventDelay,
			b_data_out => b_data_from_eventDelay,
			b_push_out => b_push_from_eventDelay,
			c_data_out => c_data_from_eventDelay,
			c_push_out => c_push_from_eventDelay,
            time_out => time_from_eventDelay,
            push_event_out => push_from_eventDelay
        );


    event_scheduler_instance: event_scheduler
        port map(
            clk => slow_hlc_clk,
            rst => rst,
            hold => hit_deadline,
            time_in => time_from_time_unit,
			a_data_in => a_data_from_eventDelay,
			a_push_in => a_push_from_eventDelay,
			b_data_in => b_data_from_eventDelay,
			b_push_in => b_push_from_eventDelay,
			c_data_in => c_data_from_eventDelay,
			c_push_in => c_push_from_eventDelay,
            push_event_in => push_from_eventDelay,
            time_out => time_from_eventScheduler,
			a_data_out => a_data_from_eventScheduler,
			a_push_out => a_push_from_eventScheduler,
			b_data_out => b_data_from_eventScheduler,
			b_push_out => b_push_from_eventScheduler,
			c_data_out => c_data_from_eventScheduler,
			c_push_out => c_push_from_eventScheduler,
            push_event_out => push_from_eventScheduler,
            lost_data => lost_data
        );



    time_unit_instance: time_unit
            port map (
                clk => slow_hlc_clk,
                rst => rst,
                time_in => time_from_extInterface,
                push => push_from_new_input_check,
                time_out => time_from_time_unit
            );


    scheduler_instance: scheduler
        port map(
            clk => slow_hlc_clk,
            rst => rst,
            time_in => time_from_time_unit,
            time_last_deadline_out => time_from_scheduler,
            hit_deadline_out => hit_deadline
        );

    hlQInterface_instance: hlQInterface
        port map (
            clk => clk,
            rst => rst,
            time_for_event => time_from_eventScheduler,
            time_for_deadline => time_from_scheduler,
            push_event => push_from_eventScheduler,
            push_deadline => hit_deadline,
			a_data_in => a_data_from_eventScheduler,
			a_en_in => a_push_from_eventScheduler,
			b_data_in => b_data_from_eventScheduler,
			b_en_in => b_push_from_eventScheduler,
			c_data_in => c_data_from_eventScheduler,
			c_en_in => c_push_from_eventScheduler,
			a_data_out => a_data_out,
			a_en_out => a_en_out,
			b_data_out => b_data_out,
			b_en_out => b_en_out,
			c_data_out => c_data_out,
			c_en_out => c_en_out,
			plus_op_en_out => plus_op_en_out,
			minus_op_en_out => minus_op_en_out,
			mult_op_en_out => mult_op_en_out,
			func_abs_en_out => func_abs_en_out,
			func_sqrt_en_out => func_sqrt_en_out,
			counter_en_out => counter_en_out,
            time_out => time_data_out,
            push_out => push_data_in_query
        );


    process(clk, rst) begin
        if (rst = '1') then
            -- Reset Phase
            slow_hlc_clk <= '0';
            hlc_clk_count <= 0;
        elsif rising_edge(clk) then
            -- Logic Phase: Raise Slow Clock Signal Every Fourth Cycle
            hlc_clk_count <= (hlc_clk_count + 1) mod 4;
            if hlc_clk_count = 3 then
                slow_hlc_clk <= '1';
            else
                slow_hlc_clk <= '0';
            end if;
         end if;
    end process;



end mixed;
