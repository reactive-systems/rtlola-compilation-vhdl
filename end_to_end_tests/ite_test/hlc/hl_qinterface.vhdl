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
		a_data_in : in signed(7 downto 0);
		a_en_in : in std_logic;
		b_data_in : in signed(15 downto 0);
		b_en_in : in std_logic;
		val_data_in : in std_logic;
		val_en_in : in std_logic;
		a_data_out : out signed(7 downto 0);
		a_en_out : out std_logic;
		b_data_out : out signed(15 downto 0);
		b_en_out : out std_logic;
		val_data_out : out std_logic;
		val_en_out : out std_logic;
		c_en_out : out std_logic;
		d_en_out : out std_logic;
		e_en_out : out std_logic;
		counter_en_out : out std_logic;
        time_out : out unsigned(63 downto 0);
        push_out : out std_logic
    );
end hlQInterface;

--* Streams and their Activation Conditions:
--* Input Streams 
--* - a @ { a }
--* - b @ { b }
--* - val @ { val }
--* Event-based Output Streams 
--* - c @ { b }
--* - d @ { a, b, val }
--* - e @ { a, b, val }
--* Periodic Output Streams 
--* - counter @ 1Hz
--* Resulting Deadline Array
--* || counter ||

architecture behavioral of hlQInterface is

    -- Internal Signal Declarations
    signal clock_state_machine : integer;
    signal push_to_queue : std_logic;
	signal a_data_push : signed(7 downto 0);
	signal a_en_push: std_logic;
	signal b_data_push : signed(15 downto 0);
	signal b_en_push: std_logic;
	signal val_data_push : std_logic;
	signal val_en_push: std_logic;
	signal c_en_push : std_logic;
	signal d_en_push : std_logic;
	signal e_en_push : std_logic;
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
			val_data_push <= '0';
			val_en_push <= '0';
			c_en_push <= '0';
			d_en_push <= '0';
			e_en_push <= '0';
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
				--* val @ { val }
				val_en_push <= '0';
				--* c @ { b }
				c_en_push <= '0';
				--* d @ { a, b, val }
				d_en_push <= '0';
				--* e @ { a, b, val }
				e_en_push <= '0';
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
				--* val @ { val }
				val_data_push <= val_data_in;
				val_en_push <= val_en_in;
				--* c @ { b }
				c_en_push <= '1' and b_en_in;
				--* d @ { a, b, val }
				d_en_push <= '1' and a_en_in and b_en_in and val_en_in;
				--* e @ { a, b, val }
				e_en_push <= '1' and a_en_in and b_en_in and val_en_in;
				--* counter @ 1Hz
				counter_en_push <= '0';
            else
                -- Enable No Stream
                push_to_queue <= '0';
				a_en_push <= '0';
				b_en_push <= '0';
				val_en_push <= '0';
				c_en_push <= '0';
				d_en_push <= '0';
				e_en_push <= '0';
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
	val_data_out <= val_data_push;
	val_en_out <= val_en_push;
	c_en_out <= c_en_push;
	d_en_out <= d_en_push;
	e_en_out <= e_en_push;
	counter_en_out <= counter_en_push;

end behavioral;
