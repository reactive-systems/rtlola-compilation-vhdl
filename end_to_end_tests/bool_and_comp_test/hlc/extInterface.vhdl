library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

entity extInterface is
    port(
        clk, rst: in std_logic;
        time_in : in std_logic_vector(63 downto 0);
		a_data_in : in std_logic;
		a_push_in : in std_logic;
		b_data_in : in std_logic;
		b_push_in : in std_logic;
		ID_data_in : in std_logic_vector(7 downto 0);
		ID_push_in : in std_logic;
		a_data_out : out std_logic;
		a_push_out : out std_logic;
		b_data_out : out std_logic;
		b_push_out : out std_logic;
		ID_data_out : out signed(7 downto 0);
		ID_push_out : out std_logic;
        time_out : out unsigned(63 downto 0)
    );
end extInterface;

--* Input Streams and their Types in the Specification: 
--* - input a : Bool
--* - input b : Bool
--* - input ID : Int8

architecture behavioral of extInterface is

    -- Internal Signal Declarations
    signal time_converted : unsigned(63 downto 0);
	signal a_parsed : std_logic;
	signal a_push_delayed : std_logic;
	signal b_parsed : std_logic;
	signal b_push_delayed : std_logic;
	signal ID_parsed : signed(7 downto 0);
	signal ID_push_delayed : std_logic;

begin

    process(clk, rst) begin
        if rst = '1' then
            -- Reset Phase
            time_converted <= to_unsigned(0, time_converted'length);
			a_parsed <= '0';
			a_push_delayed <= '0';
			b_parsed <= '0';
			b_push_delayed <= '0';
			ID_parsed <= (others => '0');
			ID_push_delayed <= '0';
        elsif rising_edge(clk) then
            -- Logic Phase: Convert Input in Numeric Types
            time_converted <= unsigned(time_in);
			--* input a : Bool
			a_parsed <= a_data_in;
			a_push_delayed <= a_push_in;
			--* input b : Bool
			b_parsed <= b_data_in;
			b_push_delayed <= b_push_in;
			--* input ID : Int8
			ID_parsed <= signed(ID_data_in);
			ID_push_delayed <= ID_push_in;
        end if;
    end process;

    time_out <= time_converted;
	a_data_out <= a_parsed;
	a_push_out <= a_push_delayed;
	b_data_out <= b_parsed;
	b_push_out <= b_push_delayed;
	ID_data_out <= ID_parsed;
	ID_push_out <= ID_push_delayed;

end behavioral;