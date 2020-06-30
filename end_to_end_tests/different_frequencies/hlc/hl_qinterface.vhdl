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
		a_data_in : in signed(31 downto 0);
		a_en_in : in std_logic;
		a_data_out : out signed(31 downto 0);
		a_en_out : out std_logic;
		b_en_out : out std_logic;
		c_en_out : out std_logic;
		d_en_out : out std_logic;
        time_out : out unsigned(63 downto 0);
        push_out : out std_logic
    );
end hlQInterface;

--* Streams and their Activation Conditions:
--* Input Streams 
--* - a @ { a }
--* Event-based Output Streams 
--* Periodic Output Streams 
--* - b @ 10Hz
--* - c @ 5Hz
--* - d @ 5/2Hz
--* Resulting Deadline Array
--* || b | b, c | b | b, c, d ||

architecture behavioral of hlQInterface is

    -- Internal Signal Declarations
    signal clock_state_machine : integer;
    signal push_to_queue : std_logic;
	signal a_data_push : signed(31 downto 0);
	signal a_en_push: std_logic;
	signal b_en_push : std_logic;
	signal c_en_push : std_logic;
	signal d_en_push : std_logic;
	signal b_en_array : bit_array(3 downto 0);
	signal c_en_array : bit_array(3 downto 0);
	signal d_en_array : bit_array(3 downto 0);
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
			b_en_push <= '0';
			c_en_push <= '0';
			d_en_push <= '0';
            -- Initialize Deadline Arrays
            --* Deadline Array
            --* || b | b, c | b | b, c, d ||
			b_en_array(0) <= '1';
			c_en_array(0) <= '0';
			d_en_array(0) <= '0';
			b_en_array(1) <= '1';
			c_en_array(1) <= '1';
			d_en_array(1) <= '0';
			b_en_array(2) <= '1';
			c_en_array(2) <= '0';
			d_en_array(2) <= '0';
			b_en_array(3) <= '1';
			c_en_array(3) <= '1';
			d_en_array(3) <= '1';
        elsif (rising_edge(clk)) then
            clock_state_machine <= (clock_state_machine + 1) mod 4;
            if push_deadline = '1' and clock_state_machine = 0 then
                -- Deadline Handling
                push_to_queue <= '1';
                last_deadline_id <= (last_deadline_id + 1) mod 4;
                time_to_queue <= time_for_deadline;
				--* a @ { a }
				a_en_push <= '0';
				--* b @ 10Hz
				b_en_push <= b_en_array(last_deadline_id);
				--* c @ 5Hz
				c_en_push <= c_en_array(last_deadline_id);
				--* d @ 5/2Hz
				d_en_push <= d_en_array(last_deadline_id);
            elsif push_event = '1' and clock_state_machine = 2 then
                -- Event Handling
                push_to_queue <= '1';
                time_to_queue <= time_for_event;
				--* a @ { a }
				a_data_push <= a_data_in;
				a_en_push <= a_en_in;
				--* b @ 10Hz
				b_en_push <= '0';
				--* c @ 5Hz
				c_en_push <= '0';
				--* d @ 5/2Hz
				d_en_push <= '0';
            else
                -- Enable No Stream
                push_to_queue <= '0';
				a_en_push <= '0';
				b_en_push <= '0';
				c_en_push <= '0';
				d_en_push <= '0';
            end if;
        end if;
    end process;

    push_out <= push_to_queue;
    time_out <= time_to_queue;
	a_data_out <= a_data_push;
	a_en_out <= a_en_push;
	b_en_out <= b_en_push;
	c_en_out <= c_en_push;
	d_en_out <= d_en_push;

end behavioral;
