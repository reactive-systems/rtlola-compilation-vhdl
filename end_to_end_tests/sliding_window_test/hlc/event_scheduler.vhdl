library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity event_scheduler is
    port(
        clk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
        hold : in std_logic;
		a_data_in : in signed(31 downto 0);
		a_push_in : in std_logic;
        push_event_in : std_logic;
        time_out : out unsigned(63 downto 0);
        push_event_out : out std_logic;
		a_data_out : out signed(31 downto 0);
		a_push_out : out std_logic;
        lost_data : out std_logic
    );
end event_scheduler;

architecture behavioral of event_scheduler is

    signal time_reg : unsigned64_array(1 downto 0);
    signal push_event_reg : bit_array(1 downto 0);
	signal a_data : signed32_array(1 downto 0);
	signal a_push : bit_array(1 downto 0);
    signal hold_reg : std_logic;
    signal lost_data_reg : std_logic;

    begin

    process(clk, rst) begin
        if (rst = '1') then
            time_reg(time_reg'high downto 0) <= (others => (others => '0'));
            push_event_reg(push_event_reg'high downto 0)<= (others => '0');
			a_data(a_data'high downto 0) <= (others => (others => '0'));
			a_push(a_push'high downto 0) <= (others => '0');
            lost_data_reg <= '0';
        elsif (rising_edge(clk)) then
            if (hold = '1') then
                if (push_event_in = '1' and push_event_reg(0) = '0') then
                    lost_data_reg <= '0';
                    push_event_reg(0) <= '1';
						a_data(0) <= a_data_in;
						a_push(0) <= a_push_in;
                else
                    lost_data_reg <= push_event_in;
                end if;
            else
                time_reg <= time_reg(time_reg'high - 1 downto 0) & time_in;
                push_event_reg <= push_event_reg(push_event_reg'high - 1 downto 0) & push_event_in;
					a_data <= a_data(a_data'high - 1 downto 0) & a_data_in;
					a_push <= a_push(a_push'high - 1 downto 0) & a_push_in;
                lost_data_reg <= '0';
            end if;
        end if;
    end process;

    time_out <= time_reg(1);
    push_event_out <= push_event_reg(1);
	a_data_out <= a_data(1);
	a_push_out <= a_push(1);
    lost_data <= lost_data_reg;

end behavioral;