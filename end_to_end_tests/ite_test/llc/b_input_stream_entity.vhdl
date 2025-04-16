library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

--* Input Stream in the Specification
--* input b : Int16
--* Input Dependencies:
--* Stream Lookups:
--* - c: 0
--* - d: 0
--* - e: 0
--* Storage Requirement: 0

entity b_input_stream_entity is
    port (
        clk, upd, rst : in std_logic;
        data_in : in signed(15 downto 0);
        data_out : out signed16_array(0 downto 0);
        data_valid_out : out bit_array(0 downto 0);
        done_out : out std_logic
    );
end b_input_stream_entity;

architecture behavioral of b_input_stream_entity is

    -- Internal Signal Declarations
    signal done : std_logic;
    signal data : signed16_array(0 downto 0);
    signal data_valid : bit_array(0 downto 0);

    begin

    process (clk, rst) begin
        if (rst='1') then
            -- Reset Phase
            data(data'high downto 0) <= (others => (others => '0'));
            data_valid(data_valid'high downto 0) <= (others => '0');
            done <= '0';
        elsif (rising_edge(clk)) then
            -- Logic Phase
            if (upd = '1' and done = '0') then
                -- Register Update
                data <= data(data'high-1 downto 0) & data_in;
                data_valid <= data_valid(data_valid'high-1 downto 0) & '1';
                done <= '1';
            elsif (upd = '0') then
                -- Reset done Signal
                done <= '0';
            end if;
        end if;
    end process;

    -- Mapping Register to Output Wires
    data_out <= data;
    data_valid_out <= data_valid;
    done_out <= done;

end behavioral;
