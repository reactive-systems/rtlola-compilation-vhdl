library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

--* Sliding window in the specification:
--* a.aggregate(over: 0.3 s, using: avg)
--* Source Stream: av_s @10Hz
--* Number of Buckets: 3
--* Time per Bucket: 0.09999999999999999s
--* Input Type: Int32
--* Return Type: Option<Int32>

entity a_avg_2_sliding_window_entity is
    port (
        clk, rst : in std_logic;
        evict, upd, request : in std_logic;
        time_in : in unsigned(63 downto 0);
        data_in : in signed(31 downto 0);
        data_out : out signed(31 downto 0);
        data_valid_out : out std_logic;
        evict_done_out : out std_logic;
        upd_done_out : out std_logic;
        request_done_out : out std_logic
    );
end a_avg_2_sliding_window_entity;

architecture behavioral of a_avg_2_sliding_window_entity is

    -- Internal Signal Declarations
    signal last_ts_before_upd : unsigned(63 downto 0);
    signal sw_data : signed(31 downto 0);
    signal sw_data_valid : std_logic;
    -- Done Signals
    signal evict_done : std_logic;
    signal upd_done : std_logic;
    signal request_done : std_logic;
    -- Buckets
    signal ts_buckets : unsigned64_array(2 downto 0);
    signal count_buckets : signed32_array(2 downto 0);
	signal sum_buckets : signed32_array(2 downto 0);
    signal data_valid_buckets : bit_array(2 downto 0);

    begin

    process (clk, rst) begin
        if (rst='1') then
            -- Reset Phase
            evict_done <= '0';
            upd_done <= '0';
            request_done <= '0';
            sw_data <= to_signed(0, sw_data'length);
            sw_data_valid <= '0';
            last_ts_before_upd <= to_unsigned(0, last_ts_before_upd'length);
            -- Reset Buckets
            ts_buckets(ts_buckets'high downto 0) <= (others => (others => '0'));
            data_valid_buckets(data_valid_buckets'high downto 0) <= (others => '0');
            count_buckets(count_buckets'high downto 0) <= (others => to_signed(0, sw_data'length));
			sum_buckets(sum_buckets'high downto 0) <= (others => to_signed(0, sw_data'length));
        elsif (rising_edge(clk)) then
            -- Logic Phase
            if (evict = '1' and evict_done = '0') then
                -- Evict Case: New TimeStamp
                if time_in > last_ts_before_upd then
                    -- Update Timestamp
                    last_ts_before_upd <= last_ts_before_upd + to_unsigned(100000000, last_ts_before_upd'length);
                    -- Create New Bucket and Shift Bucket Array
                    ts_buckets <= ts_buckets(ts_buckets'high-1 downto 0) & (last_ts_before_upd + to_unsigned(100000000, ts_buckets(0)'length));
                    data_valid_buckets <= data_valid_buckets(data_valid_buckets'high-1 downto 0) & '0';
                    count_buckets <= count_buckets(count_buckets'high-1 downto 0) & to_signed(0, sw_data'length);
					sum_buckets <= sum_buckets(sum_buckets'high-1 downto 0) & to_signed(0, sw_data'length);
                else
                    evict_done <= '1';
                end if;
            elsif (upd = '1' and upd_done = '0') then
                -- Update Case: Map New Input and Update Last Buckets Entry
                data_valid_buckets(0) <= '1';
				count_buckets(0) <= count_buckets(0) + to_signed(1, count_buckets(0)'length);
				sum_buckets(0) <= sum_buckets(0) + data_in;
                upd_done <= '1';
            elsif (request = '1' and request_done = '0') then
                -- Request Case: Finalize Buckets
                sw_data <= ((sum_buckets(0) + sum_buckets(1) + sum_buckets(2)) / (count_buckets(0) + count_buckets(1) + count_buckets(2)));
                sw_data_valid <= data_valid_buckets(0) or data_valid_buckets(1) or data_valid_buckets(2);
                request_done <= '1';
            elsif (evict ='0' and upd = '0' and request = '0') then
                evict_done <= '0';
                upd_done <= '0';
                request_done <= '0';
            end if;
        end if;
    end process;

    -- Map Internal Signals to Output Wires
    data_out <= sw_data;
    data_valid_out <= sw_data_valid;
    evict_done_out <= evict_done;
    upd_done_out <= upd_done;
    request_done_out <= request_done;

end behavioral;
