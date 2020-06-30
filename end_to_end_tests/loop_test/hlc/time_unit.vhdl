library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity time_unit is
    port(
        clk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
        push : in std_logic;
        time_out : out unsigned(63 downto 0)
    );
end time_unit;

architecture behavioral of time_unit is

    signal last_time : unsigned(63 downto 0);

begin

    process(clk, rst) begin
        if rst = '1' then
            last_time <= to_unsigned(0, last_time'length);
        elsif rising_edge(clk) then
            if push = '1' and last_time < time_in then
                last_time <= time_in;
            end if;
        end if;
    end process;

    time_out <= last_time;

end behavioral;