library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity hlQInterface is
    port (
        clk, rst : in std_logic;
        time_for_event : in unsigned(63 downto 0);
        time_for_deadline : in unsigned(63 downto 0);
        push_event : std_logic;
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
end hlQInterface;

--* Streams and their Activation Conditions:
--* Input Streams 
--* - a @ { a }
--* - b @ { b }
--* - c @ { c }
--* Event-based Output Streams 
--* - plus_op @ { a, b }
--* - minus_op @ { a, b }
--* - mult_op @ { a, b }
--* - func_abs @ { b }
--* - func_sqrt @ { c }
--* Periodic Output Streams 
--* - counter @ 1Hz
--* Resulting Deadline Array
--* || counter ||

architecture behavioral of hlQInterface is

    -- Internal Signal Declarations
    signal clock_state_machine : integer;
    signal push_to_queue : std_logic;
	signal a_data_push : sfixed(8 downto -23);
	signal a_en_push: std_logic;
	signal b_data_push : sfixed(8 downto -23);
	signal b_en_push: std_logic;
	signal c_data_push : sfixed(8 downto -23);
	signal c_en_push: std_logic;
	signal plus_op_en_push : std_logic;
	signal minus_op_en_push : std_logic;
	signal mult_op_en_push : std_logic;
	signal func_abs_en_push : std_logic;
	signal func_sqrt_en_push : std_logic;
	signal counter_en_push : std_logic;
	signal counter_en_array : bit_array(0 downto 0);
    signal last_deadline_id : integer;
    signal time_to_queue : unsigned(63 downto 0);

begin

    process(clk, rst) begin
        if (rst = '1') then
            -- Reset Phase
            clock_state_machine <= 0;
            time_to_queue <= (others => '0');
            push_to_queue <= '0';
            last_deadline_id <= 0;
			a_data_push <= (others => '0');
			a_en_push <= '0';
			b_data_push <= (others => '0');
			b_en_push <= '0';
			c_data_push <= (others => '0');
			c_en_push <= '0';
			plus_op_en_push <= '0';
			minus_op_en_push <= '0';
			mult_op_en_push <= '0';
			func_abs_en_push <= '0';
			func_sqrt_en_push <= '0';
			counter_en_push <= '0';
            -- Initialize Deadline Arrays
            --* Deadline Array
            --* || counter ||
			counter_en_array(0) <= '1';
        elsif (rising_edge(clk)) then
            clock_state_machine <= (clock_state_machine + 1) mod 4;
            if push_deadline = '1' and clock_state_machine = 0 then
                -- Deadline Handling
                push_to_queue <= '1';
                last_deadline_id <= (last_deadline_id + 1) mod 1;
                time_to_queue <= time_for_deadline;
				--* a @ { a }
				a_en_push <= '0';
				--* b @ { b }
				b_en_push <= '0';
				--* c @ { c }
				c_en_push <= '0';
				--* plus_op @ { a, b }
				plus_op_en_push <= '0';
				--* minus_op @ { a, b }
				minus_op_en_push <= '0';
				--* mult_op @ { a, b }
				mult_op_en_push <= '0';
				--* func_abs @ { b }
				func_abs_en_push <= '0';
				--* func_sqrt @ { c }
				func_sqrt_en_push <= '0';
				--* counter @ 1Hz
				counter_en_push <= counter_en_array(last_deadline_id);
            elsif push_event = '1' and clock_state_machine = 2 then
                -- Event Handling
                push_to_queue <= '1';
                time_to_queue <= time_for_event;
				--* a @ { a }
				a_data_push <= a_data_in;
				a_en_push <= a_en_in;
				--* b @ { b }
				b_data_push <= b_data_in;
				b_en_push <= b_en_in;
				--* c @ { c }
				c_data_push <= c_data_in;
				c_en_push <= c_en_in;
				--* plus_op @ { a, b }
				plus_op_en_push <= '1' and a_en_in and b_en_in;
				--* minus_op @ { a, b }
				minus_op_en_push <= '1' and a_en_in and b_en_in;
				--* mult_op @ { a, b }
				mult_op_en_push <= '1' and a_en_in and b_en_in;
				--* func_abs @ { b }
				func_abs_en_push <= '1' and b_en_in;
				--* func_sqrt @ { c }
				func_sqrt_en_push <= '1' and c_en_in;
				--* counter @ 1Hz
				counter_en_push <= '0';
            else
                -- Enable No Stream
                push_to_queue <= '0';
				a_en_push <= '0';
				b_en_push <= '0';
				c_en_push <= '0';
				plus_op_en_push <= '0';
				minus_op_en_push <= '0';
				mult_op_en_push <= '0';
				func_abs_en_push <= '0';
				func_sqrt_en_push <= '0';
				counter_en_push <= '0';
            end if;
        end if;
    end process;

    push_out <= push_to_queue;
    time_out <= time_to_queue;
	a_data_out <= a_data_push;
	a_en_out <= a_en_push;
	b_data_out <= b_data_push;
	b_en_out <= b_en_push;
	c_data_out <= c_data_push;
	c_en_out <= c_en_push;
	plus_op_en_out <= plus_op_en_push;
	minus_op_en_out <= minus_op_en_push;
	mult_op_en_out <= mult_op_en_push;
	func_abs_en_out <= func_abs_en_push;
	func_sqrt_en_out <= func_sqrt_en_push;
	counter_en_out <= counter_en_push;

end behavioral;
