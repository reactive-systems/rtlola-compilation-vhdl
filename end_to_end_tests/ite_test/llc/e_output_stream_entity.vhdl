library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;
use work.my_math_pkg.all;

--* Output Stream in the Specification
--* output e : Int8 := if (b < 3) then (if val then a else cast(b) + 4) else cast(b)
--* Input Dependencies:
--* Storage Requirement: 0
--* Output Dependencies:
--* Stream Lookups
--* - a of Type Int8: 0
--* - b of Type Int16: 0
--* - val of Type Bool: 0


entity e_output_stream_entity is 
	port (
		clk, pe, eval, rst : in std_logic;
			a_0 : in signed(7 downto 0);
			a_data_valid_0 : in std_logic;
			b_0 : in signed(15 downto 0);
			b_data_valid_0 : in std_logic;
			val_0 : in std_logic;
			val_data_valid_0 : in std_logic;
		data_out : out signed8_array(0 downto 0);
		data_valid_out : out bit_array(0 downto 0);
		pe_done_out : out std_logic;
		eval_done_out : out std_logic
	);
end e_output_stream_entity;

architecture behavioral of e_output_stream_entity is

    signal pe_done : std_logic;
    signal eval_done : std_logic;
    signal data : signed8_array(0 downto 0);
    signal data_valid : bit_array(0 downto 0);

    begin

    process (clk, rst)
        -- temporal variables
		variable temp_0: signed(15 downto 0) := (others => '0');
		variable temp_1: signed(15 downto 0) := (others => '0');
		variable temp_2: std_logic := '0';
		variable temp_3: std_logic := '0';
		variable temp_4: signed(7 downto 0) := (others => '0');
		variable temp_5: signed(15 downto 0) := (others => '0');
		variable temp_6: std_logic_vector(7 downto 0) := (others => '0');
		variable temp_7: signed(7 downto 0) := (others => '0');
		variable temp_8: signed(7 downto 0) := (others => '0');
		variable temp_9: signed(7 downto 0) := (others => '0');
		variable temp_10: signed(7 downto 0) := (others => '0');
		variable temp_11: signed(15 downto 0) := (others => '0');
		variable temp_12: std_logic_vector(7 downto 0) := (others => '0');
		variable temp_13: signed(7 downto 0) := (others => '0');
		variable temp_14: signed(7 downto 0) := (others => '0');
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
				--* temp_0 := b 
				temp_0 := b_0;
				temp_1 := to_signed(3, 16);
				--* temp_2 := (b < 3) 
				temp_2 := to_std_logic(temp_0 < temp_1);
				--* temp_14 := if (b < 3) then (if val then a else cast(b) + 4) else cast(b) 
				if temp_2 = '1' then
				--* temp_3 := val 
				temp_3 := val_0;
				--* temp_8 := if val then a else cast(b) 
				if temp_3 = '1' then
				--* temp_4 := a 
				temp_4 := a_0;
				temp_8 := temp_4;
				else
				--* temp_5 := b 
				temp_5 := b_0;
				temp_6 := std_logic_vector(temp_5(7 downto 0));
				temp_7 := signed(temp_6);
				temp_8 := temp_7;
				end if;
				temp_9 := to_signed(4, 8);
				--* temp_10 := (if val then a else cast(b) + 4) 
				temp_10 := temp_8 + temp_9;
				temp_14 := temp_10;
				else
				--* temp_11 := b 
				temp_11 := b_0;
				temp_12 := std_logic_vector(temp_11(7 downto 0));
				temp_13 := signed(temp_12);
				temp_14 := temp_13;
				end if;
				updt := temp_14;
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
