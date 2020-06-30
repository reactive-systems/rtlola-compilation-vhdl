library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

package array_type_pkg is 
    type signed8_array is array (natural range <>) of signed(7 downto 0);
    type signed16_array is array (natural range <>) of signed(15 downto 0);
    type signed32_array is array (natural range <>) of signed(31 downto 0);
    type signed64_array is array (natural range <>) of signed(63 downto 0);
    type unsigned8_array is array (natural range <>) of unsigned(7 downto 0);
    type unsigned16_array is array (natural range <>) of unsigned(15 downto 0);
    type unsigned32_array is array (natural range <>) of unsigned(31 downto 0);
    type unsigned64_array is array (natural range <>) of unsigned(63 downto 0);
    type bit_array is array (natural range <>) of std_logic;
    type sfixed16_array is array (natural range <>) of sfixed(4 downto -11);
    type sfixed32_array is array (natural range <>) of sfixed(8 downto -23);
    type sfixed64_array is array (natural range <>) of sfixed(11 downto -52);
    function sel(in_sig, d, valid : std_logic) return std_logic;
    function sel(in_sig, d : signed; valid : std_logic) return signed;
    function sel(in_sig, d : unsigned; valid : std_logic) return unsigned;
    function sel(in_sig, d: sfixed; valid: std_logic) return sfixed;
    function to_std_logic(b : boolean) return std_logic;
end;

package body array_type_pkg is
    function sel(in_sig, d, valid : std_logic) return std_logic is
    begin
        if (valid = '1') then
            return in_sig;
        else
            return d;
        end if;
    end sel;

    function sel(in_sig, d : signed; valid: std_logic) return signed is
    begin
        if (valid = '1') then
            return in_sig;
        else
            return d;
        end if;
    end sel;

    function sel(in_sig, d : unsigned; valid: std_logic) return unsigned is
    begin
        if (valid = '1') then
            return in_sig;
        else
            return d;
        end if;
    end sel;

    function sel(in_sig, d: sfixed; valid: std_logic) return sfixed is
    begin
        if (valid = '1') then 
            return in_sig;
        else
            return d;
        end if;
    end sel;

    function to_std_logic(b : boolean) return std_logic is
    begin
        if b then
            return '1';
        else
            return '0';
        end if;
    end to_std_logic;
end array_type_pkg;
