library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;
use work.my_math_pkg.all;

--* Output Stream in the Specification
--* output counter : Int64 := (counter.offset(by: neg1).defaults(to: 0) + 1)
--* Input Dependencies:
--* Stream Lookups:
--* - counter: -1
--* Storage Requirement: 1
--* Output Dependencies:
--* Stream Lookups
--* - counter of Type Int64: -1


entity counter_output_stream_entity is 
	port (
		clk, pe, eval, rst : in std_logic;
			counter_neg1 : in signed(63 downto 0);
			counter_data_valid_neg1 : in std_logic;
		data_out : out signed64_array(1 downto 0);
		data_valid_out : out bit_array(1 downto 0);
		pe_done_out : out std_logic;
		eval_done_out : out std_logic
	);
end counter_output_stream_entity;

architecture behavioral of counter_output_stream_entity is

    signal pe_done : std_logic;
    signal eval_done : std_logic;
    signal data : signed64_array(1 downto 0);
    signal data_valid : bit_array(1 downto 0);

    begin

    process (clk, rst)
        -- temporal variables
		variable temp_0: signed(63 downto 0) := (others => '0');
		variable temp_1: signed(63 downto 0) := (others => '0');
		variable temp_2: signed(63 downto 0) := (others => '0');
		variable temp_3: signed(63 downto 0) := (others => '0');
		variable temp_4: signed(63 downto 0) := (others => '0');
	    variable updt : signed(63 downto 0) := (others => '0');
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
				--* temp_0 := counter.offset(by: neg1)
				temp_0 := counter_neg1;
				temp_1 := to_signed(0, 64);
				--* temp_2 := counter.offset(by: neg1).defaults(to: 0) 
				temp_2 := sel(temp_0, temp_1, counter_data_valid_neg1);
				temp_3 := to_signed(1, 64);
				--* temp_4 := (counter.offset(by: neg1).defaults(to: 0) + 1) 
				temp_4 := temp_2 + temp_3;
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
