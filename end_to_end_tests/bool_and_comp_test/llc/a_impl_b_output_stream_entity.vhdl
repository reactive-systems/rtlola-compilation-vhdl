library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;
use work.my_math_pkg.all;

--* Output Stream in the Specification
--* output a_impl_b : Bool := ((not a) or b)
--* Input Dependencies:
--* Stream Lookups:
--* - a_equiv_b: 0
--* Storage Requirement: 0
--* Output Dependencies:
--* Stream Lookups
--* - a of Type Bool: 0
--* - b of Type Bool: 0


entity a_impl_b_output_stream_entity is 
	port (
		clk, pe, eval, rst : in std_logic;
			a_0 : in std_logic;
			a_data_valid_0 : in std_logic;
			b_0 : in std_logic;
			b_data_valid_0 : in std_logic;
		data_out : out bit_array(0 downto 0);
		data_valid_out : out bit_array(0 downto 0);
		pe_done_out : out std_logic;
		eval_done_out : out std_logic
	);
end a_impl_b_output_stream_entity;

architecture behavioral of a_impl_b_output_stream_entity is

    signal pe_done : std_logic;
    signal eval_done : std_logic;
    signal data : bit_array(0 downto 0);
    signal data_valid : bit_array(0 downto 0);

    begin

    process (clk, rst)
        -- temporal variables
		variable temp_0: std_logic := '0';
		variable temp_1: std_logic := '0';
		variable temp_2: std_logic := '0';
		variable temp_3: std_logic := '0';
	    variable updt : std_logic := '0';
    begin
	    if (rst='1') then
	        -- Reset Phase
		    data(data'high downto 0) <= (others => '0');
		    data_valid(data_valid'high downto 0) <= (others => '0');
		    pe_done <= '0';
		    eval_done <= '0';
	    elsif (rising_edge(clk)) then
	        -- Logic Phase
	        if (pe = '1' and pe_done = '0') then
	            -- Pseudo Evaluation
                data <= data(data'high-1 downto 0) & '0';
                data_valid <= data_valid(data_valid'high-1 downto 0) & '0';
                pe_done <= '1';
		    elsif (eval = '1' and eval_done = '0') then
				-- Evaluation
				--* temp_0 := a 
				temp_0 := a_0;
				--* temp_1 := (not a) 
				temp_1 := not temp_0;
				--* temp_2 := b 
				temp_2 := b_0;
				--* temp_3 := ((not a) or b) 
				temp_3 := temp_1 or temp_2;
				updt := temp_3;
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
