library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity event_delay is
    port(
        clk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
		a_data_in : in signed(7 downto 0);
		a_push_in : in std_logic;
		b_data_in : in signed(7 downto 0);
		b_push_in : in std_logic;
        push_event_in : std_logic;
        time_out : out unsigned(63 downto 0);
		a_data_out : out signed(7 downto 0);
		a_push_out : out std_logic;
		b_data_out : out signed(7 downto 0);
		b_push_out : out std_logic;
        push_event_out : out std_logic
    );
end event_delay;

--* Input Streams and their Types in the Specification:
--* - a : Int8 *--
--* - b : Int8 *--

architecture behavioral of event_delay is

    -- Internal Signal Declarations
    signal time_value : unsigned(63 downto 0);
    signal push_event : std_logic;
	signal a_data_delayed : signed(7 downto 0);
	signal a_push_delayed : std_logic;
	signal b_data_delayed : signed(7 downto 0);
	signal b_push_delayed : std_logic;

    begin

    process(clk, rst) begin
        if (rst = '1') then
            -- Reset Phase
            time_value <= (others => '0');
            push_event <= '0';
			a_data_delayed <= to_signed(0, a_data_delayed'length);
			a_push_delayed <= '0';
			b_data_delayed <= to_signed(0, b_data_delayed'length);
			b_push_delayed <= '0';
        elsif (rising_edge(clk)) then
            -- Logic Phase: Map Inputs to Internal Signals to Receive a Delay
            if (push_event_in = '1') then
                time_value <= time_in;
                push_event <= push_event_in;
				a_data_delayed <= a_data_in;
				a_push_delayed <= a_push_in;
				b_data_delayed <= b_data_in;
				b_push_delayed <= b_push_in;
            else
                time_value <= (others => '0');
                push_event <= '0';
				a_data_delayed <= to_signed(0, a_data_delayed'length);
				a_push_delayed <= '0';
				b_data_delayed <= to_signed(0, b_data_delayed'length);
				b_push_delayed <= '0';
            end if;
        end if;
    end process;

    time_out <= time_value;
    push_event_out <= push_event;
	a_data_out <= a_data_delayed;
	a_push_out <= a_push_delayed;
	b_data_out <= b_data_delayed;
	b_push_out <= b_push_delayed;

end behavioral;