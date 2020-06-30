library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;
use work.my_math_pkg.all;

--* Output Stream in the Specification
--* output f : Int8 := (a + a.offset(by: neg1).defaults(to: 0))
--* Input Dependencies:
--* Stream Lookups:
--* - g: 0
--* Storage Requirement: 0
--* Output Dependencies:
--* Stream Lookups
--* - a of Type Int8: 0, -1


entity f_output_stream_entity is 
	port (
		clk, pe, eval, rst : in std_logic;
			a_0 : in signed(7 downto 0);
			a_data_valid_0 : in std_logic;
			a_neg1 : in signed(7 downto 0);
			a_data_valid_neg1 : in std_logic;
		data_out : out signed8_array(0 downto 0);
		data_valid_out : out bit_array(0 downto 0);
		pe_done_out : out std_logic;
		eval_done_out : out std_logic
	);
end f_output_stream_entity;

architecture behavioral of f_output_stream_entity is

    signal pe_done : std_logic;
    signal eval_done : std_logic;
    signal data : signed8_array(0 downto 0);
    signal data_valid : bit_array(0 downto 0);

    begin

    process (clk, rst)
        -- temporal variables
		variable temp_0: signed(7 downto 0) := (others => '0');
		variable temp_1: signed(7 downto 0) := (others => '0');
		variable temp_2: signed(7 downto 0) := (others => '0');
		variable temp_3: signed(7 downto 0) := (others => '0');
		variable temp_4: signed(7 downto 0) := (others => '0');
	    variable updt : signed(7 downto 0) := (others => '0');
    begin
	    if (rst='1') then
	        -- Reset Phase
		    data(data'high downto 0) <= (others => (others => '0'));
		    data_valid(data_valid'high downto 0) <= (others => '0');
		    pe_done <= '0';
		    eval_done <= '0';
	    elsif (rising_edge(clk)) then
	        -- Logic Phase
	        if (pe = '1' and pe_done = '0') then
	            -- Pseudo Evaluation
                data <= data(data'high-1 downto 0) & to_signed(0, updt'length);
                data_valid <= data_valid(data_valid'high-1 downto 0) & '0';
                pe_done <= '1';
		    elsif (eval = '1' and eval_done = '0') then
				-- Evaluation
				--* temp_0 := a 
				temp_0 := a_0;
				--* temp_1 := a.offset(by: neg1)
				temp_1 := a_neg1;
				temp_2 := to_signed(0, 8);
				--* temp_3 := a.offset(by: neg1).defaults(to: 0) 
				temp_3 := sel(temp_1, temp_2, a_data_valid_neg1);
				--* temp_4 := (a + a.offset(by: neg1).defaults(to: 0)) 
				temp_4 := temp_0 + temp_3;
				updt := temp_4;
			    -- Register Update
			    data(0) <= updt;
			    data_valid(0) <= '1';
			    eval_done <= '1';
			elsif (pe = '0' and eval = '0') then
                -- Reset done Signals
                pe_done <= '0';
                eval_done <= '0';
		    end if;
	    end if;
    end process;

     -- Mapping: Register to Output Wires
    data_out <= data;
    data_valid_out <= data_valid;
    pe_done_out <= pe_done;
    eval_done_out <= eval_done;

end behavioral;
