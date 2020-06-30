library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

entity check_new_input is
    port(
        clk, rst: in std_logic;
        new_input_in : in std_logic;
        new_input_out : out std_logic
    );
end check_new_input;

architecture behavioral of check_new_input is

    -- Internal Signal Declarations
    signal prev_new_input_in : std_logic;
    signal new_input : std_logic;

begin

    process(clk, rst) begin
        if rst = '1' then
            -- Reset Phase
            prev_new_input_in <= '0';
            new_input <= '0';
        elsif rising_edge(clk) then
            -- Logic Phase: Check If There Is a New Input
            if new_input_in = '1' and prev_new_input_in = '0' then
                -- Current Event Is New
                new_input <= '1';
            else
                -- No New Event
                new_input <= '0';
            end if;
            prev_new_input_in <= new_input_in;
        end if;
    end process;

    new_input_out <= new_input;

end behavioral;