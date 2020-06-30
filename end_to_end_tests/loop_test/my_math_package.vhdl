library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

package my_math_pkg is
    function my_sqrt_32(d: signed) return signed;
    function my_sqrt_32(d: unsigned) return unsigned;
    function my_sqrt_fixed_16(d: sfixed(4 downto -11)) return sfixed;
    function my_sqrt_fixed_32(d: sfixed(8 downto -23)) return sfixed;
    function my_sqrt_fixed_64(d: sfixed(11 downto -52)) return sfixed;
end;

package body my_math_pkg is

    function my_sqrt_32( d : signed ) return signed is
            variable a : signed(31 downto 0):=d;  --original input.
            variable q : signed(15 downto 0):=(others => '0');  --result.
            variable left,right,r : signed(17 downto 0):=(others => '0');  --input to adder/sub.r-remainder.
            variable i : integer:=0;
        begin
            for i in 0 to 15 loop
                right(0):='1';
                right(1):=r(17);
                right(17 downto 2):=q;
                left(1 downto 0):=a(31 downto 30);
                left(17 downto 2):=r(15 downto 0);
                a(31 downto 2):=a(29 downto 0);  --shifting by 2 bit.
                if ( r(17) = '1') then
                    r := left + right;
                else
                    r := left - right;
                end if;
                q(15 downto 1) := q(14 downto 0);
                q(0) := not r(17);
                end loop;
            return q;
        end my_sqrt_32;

    function my_sqrt_32( d : unsigned ) return unsigned is
        variable a : unsigned(31 downto 0):=d;  --original input.
        variable q : unsigned(15 downto 0):=(others => '0');  --result.
        variable left,right,r : unsigned(17 downto 0):=(others => '0');  --input to adder/sub.r-remainder.
        variable i : integer:=0;
    begin
        for i in 0 to 15 loop
            right(0):='1';
            right(1):=r(17);
            right(17 downto 2):=q;
            left(1 downto 0):=a(31 downto 30);
            left(17 downto 2):=r(15 downto 0);
            a(31 downto 2):=a(29 downto 0);  --shifting by 2 bit.
            if ( r(17) = '1') then
                r := left + right;
            else
                r := left - right;
            end if;
            q(15 downto 1) := q(14 downto 0);
            q(0) := not r(17);
        end loop;
        return q;
    end my_sqrt_32;

    -- interpret sfixed_32 bit representation as signed integer, use my_sqrt_32, and undo changes afterwards
    function my_sqrt_fixed_32(d: sfixed(8 downto -23)) return sfixed is
        variable in_as_vec : std_logic_vector(31 downto 0) := (others => '0');
        variable mul_two : std_logic_vector(31 downto 0) := (others => '0');
        variable input_for_sqrt_int_func : unsigned(31 downto 0) := (others => '0');
        variable ret_sqrt_int_func : unsigned(15 downto 0) := (others => '0');
        variable ret_sqrt_int_func_as_vec : std_logic_vector(15 downto 0) := (others => '0');
        variable ret_as_vec : std_logic_vector(31 downto 0) := (others => '0');
        variable ret : sfixed(8 downto -23) := (others => '0');
    begin
        in_as_vec := to_slv(d);
        mul_two(31 downto 1) := in_as_vec(30 downto 0);
        input_for_sqrt_int_func := unsigned(mul_two);
        ret_sqrt_int_func := my_sqrt_32(input_for_sqrt_int_func);
        ret_sqrt_int_func_as_vec := std_logic_vector(ret_sqrt_int_func);
        ret_as_vec(26 downto 11) := ret_sqrt_int_func_as_vec(15 downto 0);
        ret := sfixed(ret_as_vec);
        return ret;
    end my_sqrt_fixed_32;

    -- use my_sqrt_fixed_32 to compute my_sqrt_fixed_16 and my_sqrt_fixed_64, by casting the 16/64 bit representation to a 32 bit representation

    function my_sqrt_fixed_16(d: sfixed(4 downto -11)) return sfixed is
            variable high_cast : sfixed(8 downto -23) := (others => '0');
            variable res_func  : sfixed(8 downto -23) := (others => '0');
            variable low_cast  : sfixed(4 downto -11) := (others => '0');
        begin
            high_cast(4 downto -11) := d(4 downto -11);
            res_func := my_sqrt_fixed_32(high_cast);
            low_cast(4 downto -11) := res_func(4 downto -11);
            return low_cast;
        end my_sqrt_fixed_16;

    function my_sqrt_fixed_64(d: sfixed(11 downto -52)) return sfixed is
        variable low_cast : sfixed(8 downto -23) := (others => '0');
        variable res_func : sfixed(8 downto -23) := (others => '0');
        variable high_cast : sfixed(11 downto -52) := (others => '0');
    begin
        low_cast(8 downto -23) := d(8 downto -23);
        res_func := my_sqrt_fixed_32(low_cast);
        high_cast(8 downto -23) := res_func(8 downto -23);
        return high_cast;
    end my_sqrt_fixed_64;


end my_math_pkg;