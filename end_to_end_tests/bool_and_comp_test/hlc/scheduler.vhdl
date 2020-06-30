library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity scheduler is
    port(
        clk, rst : in std_logic;
        time_in : in unsigned(63 downto 0);
        time_last_deadline_out : out unsigned(63 downto 0);
        hit_deadline_out : out std_logic
    );
end scheduler;

--* Periodic Streams in Specification: 
--* - time_stream @ 1Hz *--
--* Resulting Hyper Period in Seconds: 1 seconds
--* Resulting Offset Array in Seconds:
--* || 1 ||

architecture behavioral of scheduler is

    -- Internal Signal Declarations
    signal time_of_next_deadline : unsigned(63 downto 0);
    signal offset_per_deadline : unsigned64_array(0 downto 0);
    signal last_deadline_id : integer;
    signal hit_deadline : std_logic;
    signal time_last_deadline : unsigned(63 downto 0);

begin

    process(clk, rst) begin
        if (rst = '1') then
            -- Reset Phase
            time_of_next_deadline <= to_unsigned(1000000000, time_of_next_deadline'length);
            last_deadline_id <= 0;
            time_last_deadline <= (others => '0');
            hit_deadline <= '0';
            -- Initialization of the Deadline Offset Array
            --* Offset Array in seconds:
            --* || 1 ||
			offset_per_deadline(0) <= to_unsigned(1000000000, offset_per_deadline(0)'length);
        elsif (rising_edge(clk)) then
            -- Logic Phase: Decision, if Arrival of a Deadline
            if (time_in >= time_of_next_deadline) then
                -- Deadline is Reached
                time_of_next_deadline <= time_of_next_deadline + offset_per_deadline(last_deadline_id);
                last_deadline_id <= (last_deadline_id + 1) mod 1;
                hit_deadline <= '1';
                time_last_deadline <= time_of_next_deadline;
            else
                -- No Deadline is Reached
                hit_deadline <= '0';
            end if;
        end if;
    end process;

    hit_deadline_out <= hit_deadline;
    time_last_deadline_out <= time_last_deadline;

end behavioral;