library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;
use work.my_math_pkg.all;

--* Output Stream in the Specification
--* output d : Int16 := if val then (cast(a) + b) else (cast(a) * b)
--* Input Dependencies:
--* Storage Requirement: 0
--* Output Dependencies:
--* Stream Lookups
--* - a of Type Int8: 0
--* - b of Type Int16: 0
--* - val of Type Bool: 0


entity d_output_stream_entity is 
	port (
		clk, pe, eval, rst : in std_logic;
			a_0 : in signed(7 downto 0);
			a_data_valid_0 : in std_logic;
			b_0 : in signed(15 downto 0);
			b_data_valid_0 : in std_logic;
			val_0 : in std_logic;
			val_data_valid_0 : in std_logic;
		data_out : out signed16_array(0 downto 0);
		data_valid_out : out bit_array(0 downto 0);
		pe_done_out : out std_logic;
		eval_done_out : out std_logic
	);
end d_output_stream_entity;

architecture behavioral of d_output_stream_entity is

    signal pe_done : std_logic;
    signal eval_done : std_logic;
    signal data : signed16_array(0 downto 0);
    signal data_valid : bit_array(0 downto 0);

    begin

    process (clk, rst)
        -- temporal variables
		variable temp_0: std_logic := '0';
		variable temp_1: signed(7 downto 0) := (others => '0');
		variable temp_2: std_logic_vector(15 downto 0) := (others => '0');
		variable temp_3: signed(15 downto 0) := (others => '0');
		variable temp_4: signed(15 downto 0) := (others => '0');
		variable temp_5: signed(15 downto 0) := (others => '0');
		variable temp_6: signed(7 downto 0) := (others => '0');
		variable temp_7: std_logic_vector(15 downto 0) := (others => '0');
		variable temp_8: signed(15 downto 0) := (others => '0');
		variable temp_9: signed(15 downto 0) := (others => '0');
		variable temp_10: signed(31 downto 0) := (others => '0');
		variable temp_11: signed(15 downto 0) := (others => '0');
		variable temp_12: signed(15 downto 0) := (others => '0');
	    variable updt : signed(15 downto 0) := (others => '0');
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
				--* temp_0 := val 
				temp_0 := val_0;
				--* temp_12 := if val then (cast(a) + b) else (cast(a) * b) 
				if temp_0 = '1' then
				--* temp_1 := a 
				temp_1 := a_0;
				temp_2(7 downto 0) := std_logic_vector(temp_1);
				temp_3 := signed(temp_2);
				--* temp_4 := b 
				temp_4 := b_0;
				--* temp_5 := (cast(a) + b) 
				temp_5 := temp_3 + temp_4;
				temp_12 := temp_5;
				else
				--* temp_6 := a 
				temp_6 := a_0;
				temp_7(7 downto 0) := std_logic_vector(temp_6);
				temp_8 := signed(temp_7);
				--* temp_9 := b 
				temp_9 := b_0;
				--* temp_11 := (cast(a) * b) 
				temp_10 := temp_8 * temp_9;
				temp_11 := temp_10(temp_11'length-1 downto 0);
				temp_12 := temp_11;
				end if;
				updt := temp_12;
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
