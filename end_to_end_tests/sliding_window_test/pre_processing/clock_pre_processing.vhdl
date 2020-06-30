library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity clock_pre_processing is
    port (
        clk: in std_logic;
        rst: in std_logic;
        sys_clk: out std_logic;
        tclk: out std_logic;
        eclk: out std_logic
    );
end entity;

architecture behaviour of clock_pre_processing is

    signal clk_reg : std_logic;
    signal eclk_reg : std_logic;
    signal tclk_reg : std_logic;

    signal eclk_count : integer;
    signal tclk_count : integer;

begin

    process(clk, rst) begin
        if rst='1' then
            clk_reg <= '0';
            tclk_count <= 4;
            eclk_count <= 14;
            eclk_reg <= '0';
            tclk_reg <= '0';
        elsif rising_edge(clk) then
            clk_reg <= not clk_reg;
            if clk_reg = '0' then
                if eclk_count = 14 then
                    eclk_reg <= '1';
                    eclk_count <= 0;
                else
                    eclk_count <= eclk_count + 1;
                end if;
                if tclk_count = 4 then
                    tclk_reg <= '1';
                    tclk_count <= 0;
                else
                    tclk_count <= tclk_count + 1;
                end if;
            else
                if eclk_count = 7 then
                    eclk_reg <= '0';
                end if;
                if tclk_count = 2 then
                    tclk_reg <= '0';
                end if;
            end if;
        end if;

    end process;

    sys_clk <= clk_reg;
    tclk <= tclk_reg;
    eclk <= eclk_reg;

end behaviour;