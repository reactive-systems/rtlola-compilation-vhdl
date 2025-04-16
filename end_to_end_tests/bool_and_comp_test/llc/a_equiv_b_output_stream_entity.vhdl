library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;
use work.my_math_pkg.all;

--* Output Stream in the Specification
--* output a_equiv_b : Bool := (a_impl_b and ((not b) or a))
--* Input Dependencies:
--* Stream Lookups:
--* - a_xor_b: 0
--* Storage Requirement: 0
--* Output Dependencies:
--* Stream Lookups
--* - a of Type Bool: 0
--* - b of Type Bool: 0
--* - a_impl_b of Type Bool: 0


entity a_equiv_b_output_stream_entity is 
	port (
		clk, pe, eval, rst : in std_logic;
			a_0 : in std_logic;
			a_data_valid_0 : in std_logic;
			b_0 : in std_logic;
			b_data_valid_0 : in std_logic;
			a_impl_b_0 : in std_logic;
			a_impl_b_data_valid_0 : in std_logic;
		data_out : out bit_array(0 downto 0);
		data_valid_out : out bit_array(0 downto 0);
		pe_done_out : out std_logic;
		eval_done_out : out std_logic
	);
end a_equiv_b_output_stream_entity;

architecture behavioral of a_equiv_b_output_stream_entity is

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
		variable temp_4: std_logic := '0';
		variable temp_5: std_logic := '0';
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
				--* temp_0 := a_impl_b 
				temp_0 := a_impl_b_0;
				--* temp_1 := b 
				temp_1 := b_0;
				--* temp_2 := (not b) 
				temp_2 := not temp_1;
				--* temp_3 := a 
				temp_3 := a_0;
				--* temp_4 := ((not b) or a) 
				temp_4 := temp_2 or temp_3;
				--* temp_5 := (a_impl_b and ((not b) or a)) 
				temp_5 := temp_0 and temp_4;
				updt := temp_5;
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
