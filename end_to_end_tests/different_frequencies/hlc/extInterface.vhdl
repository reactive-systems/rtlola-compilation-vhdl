library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

entity extInterface is
    port(
        clk, rst: in std_logic;
        time_in : in std_logic_vector(63 downto 0);
		a_data_in : in std_logic_vector(31 downto 0);
		a_push_in : in std_logic;
		a_data_out : out signed(31 downto 0);
		a_push_out : out std_logic;
        time_out : out unsigned(63 downto 0)
    );
end extInterface;

--* Input Streams and their Types in the Specification: 
--* - input a : Int32

architecture behavioral of extInterface is

    -- Internal Signal Declarations
    signal time_converted : unsigned(63 downto 0);
	signal a_parsed : signed(31 downto 0);
	signal a_push_delayed : std_logic;

begin

    process(clk, rst) begin
        if rst = '1' then
            -- Reset Phase
            time_converted <= to_unsigned(0, time_converted'length);
			a_parsed <= (others => '0');
			a_push_delayed <= '0';
        elsif rising_edge(clk) then
            -- Logic Phase: Convert Input in Numeric Types
            time_converted <= unsigned(time_in);
			--* input a : Int32
			a_parsed <= signed(a_data_in);
			a_push_delayed <= a_push_in;
        end if;
    end process;

    time_out <= time_converted;
	a_data_out <= a_parsed;
	a_push_out <= a_push_delayed;

end behavioral;