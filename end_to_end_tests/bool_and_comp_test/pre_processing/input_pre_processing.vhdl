library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity input_pre_processing is
    port(
        clk, rst: in std_logic;
        time_in : in std_logic_vector(63 downto 0);
		a_in : in std_logic;
		a_in_new_input : in std_logic;
		b_in : in std_logic;
		b_in_new_input : in std_logic;
		ID_in : in std_logic_vector(7 downto 0);
		ID_in_new_input : in std_logic;
        new_input_in : in std_logic;
        time_out : out std_logic_vector(63 downto 0);
		a_out : out std_logic;
		a_out_new_input : out std_logic;
		b_out : out std_logic;
		b_out_new_input : out std_logic;
		ID_out : out std_logic_vector(7 downto 0);
		ID_out_new_input : out std_logic;
        new_input_out : out std_logic
    );
end input_pre_processing;

architecture behavioral of input_pre_processing is

    signal new_input : std_logic;
    signal time_reg : std_logic_vector(63 downto 0);
	signal a_reg : std_logic;
	signal a_reg_new_input : std_logic;
	signal b_reg : std_logic;
	signal b_reg_new_input : std_logic;
	signal ID_reg : std_logic_vector(7 downto 0);
	signal ID_reg_new_input : std_logic;

begin

    process(clk, rst) begin
        if (rst = '1') then
            -- set default
            new_input <= '0';
            time_reg <= (others => '0');
			a_reg <= '0';
			a_reg_new_input <= '0';
			b_reg <= '0';
			b_reg_new_input <= '0';
			ID_reg <= (others => '0');
			ID_reg_new_input <= '0';
        elsif (rising_edge(clk)) then
            new_input <= new_input_in;
            time_reg <= time_in;
				a_reg <= a_in;
				a_reg_new_input <= a_in_new_input;
				b_reg <= b_in;
				b_reg_new_input <= b_in_new_input;
				ID_reg <= ID_in;
				ID_reg_new_input <= ID_in_new_input;
        end if;
    end process;

    new_input_out <= new_input;
    time_out <= time_reg;
	a_out <= a_reg;
	a_out_new_input <= a_reg_new_input;
	b_out <= b_reg;
	b_out_new_input <= b_reg_new_input;
	ID_out <= ID_reg;
	ID_out_new_input <= ID_reg_new_input;

end behavioral;