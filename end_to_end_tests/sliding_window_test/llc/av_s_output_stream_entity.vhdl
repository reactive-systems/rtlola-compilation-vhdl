library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;
use work.my_math_pkg.all;

--* Output Stream in the Specification
--* output av_s : Int32 := a.aggregate(over: 0.3s, using: avg).defaults(to: 10)
--* Input Dependencies:
--* Storage Requirement: 0
--* Output Dependencies:
--* Stream Lookups
--* - a of Type Int32: window SlidingWin(2)
--* Window Lookups:
--* - a.aggregate(over: 0.3 s, using: avg) of type Option<Int32>


entity av_s_output_stream_entity is 
	port (
		clk, pe, eval, rst : in std_logic;
			a_avg_2_sw : in signed(31 downto 0);
			a_avg_2_sw_data_valid : in std_logic;
		data_out : out signed32_array(0 downto 0);
		data_valid_out : out bit_array(0 downto 0);
		pe_done_out : out std_logic;
		eval_done_out : out std_logic
	);
end av_s_output_stream_entity;

architecture behavioral of av_s_output_stream_entity is

    signal pe_done : std_logic;
    signal eval_done : std_logic;
    signal data : signed32_array(0 downto 0);
    signal data_valid : bit_array(0 downto 0);

    begin

    process (clk, rst)
        -- temporal variables
		variable temp_0: signed(31 downto 0) := (others => '0');
		variable temp_1: signed(31 downto 0) := (others => '0');
		variable temp_2: signed(31 downto 0) := (others => '0');
	    variable updt : signed(31 downto 0) := (others => '0');
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
				--* temp_0 := a.aggregate(over: 0.3s, using: avg) 
				temp_0 := a_avg_2_sw;
				temp_1 := to_signed(10, 32);
				--* temp_2 := a.aggregate(over: 0.3s, using: avg).defaults(to: 10) 
				temp_2 := sel(temp_0, temp_1, a_avg_2_sw_data_valid);
				updt := temp_2;
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
