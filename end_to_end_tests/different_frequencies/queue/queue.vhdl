library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity queue is
    port (
        clk, rst : in std_logic;
        push : in std_logic;
        time_data_in : in unsigned(63 downto 0);
		a_data_in : in signed(31 downto 0);
		a_en_in : in std_logic;
		b_en_in : in std_logic;
		c_en_in : in std_logic;
		d_en_in : in std_logic;
        full : out std_logic;
        pop : in std_logic;
        time_data_out : out unsigned(63 downto 0);
		a_data_out : out signed(31 downto 0);
		a_en_out : out std_logic;
		b_en_out : out std_logic;
		c_en_out : out std_logic;
		d_en_out : out std_logic;
        available : out std_logic
    );
end queue;

architecture behavioral of queue is

    signal is_full : std_logic;
    signal time_data_reg : unsigned64_array(1 downto 0);
    signal time_data : unsigned(63 downto 0);
	signal a_data_reg : signed32_array(1 downto 0);
	signal a_en_reg : bit_array(1 downto 0);
	signal a_data : signed(31 downto 0);
	signal a_en: std_logic;
	signal b_en_reg : bit_array(1 downto 0);
	signal b_en : std_logic;
	signal c_en_reg : bit_array(1 downto 0);
	signal c_en : std_logic;
	signal d_en_reg : bit_array(1 downto 0);
	signal d_en : std_logic;
    signal av : std_logic;
    signal size : integer;
    signal clk_reg : std_logic;
    signal push_done : std_logic;
    signal pop_done : std_logic;

begin

    process(rst, clk) begin
        if (rst = '1') then
            is_full <= '0';
            time_data_reg(time_data_reg'high downto 0) <= (others => (others => '0'));
            time_data <= (others => '0');
			a_data_reg(a_data_reg'high downto 0) <= (others => (others => '0'));
			a_en_reg(a_en_reg'high downto 0) <= (others => '0');
			a_data <= (others => '0');
			a_en <= '0';
			b_en_reg <= (others => '0');
			b_en <= '0';
			c_en_reg <= (others => '0');
			c_en <= '0';
			d_en_reg <= (others => '0');
			d_en <= '0';
            size <= 0;
            av <= '0';
            clk_reg <= '0';
            push_done <= '0';
            pop_done <= '0';
        elsif rising_edge(clk) then
            clk_reg <= not clk_reg;
            if clk_reg = '0' then
                if push = '1' and push_done = '0' and pop = '1' and pop_done = '0' and size > 0 and size < 2 then
                    -- perform push and pop
                    time_data_reg <= time_data_reg(time_data_reg'high - 1 downto 0) & time_data_in;
					a_data_reg <= a_data_reg(a_data_reg'high - 1 downto 0) & a_data_in;
					a_en_reg <= a_en_reg(a_en_reg'high - 1 downto 0) & a_en_in;
					b_en_reg <= b_en_reg(b_en_reg'high - 1 downto 0) & b_en_in;
					c_en_reg <= c_en_reg(c_en_reg'high - 1 downto 0) & c_en_in;
					d_en_reg <= d_en_reg(d_en_reg'high - 1 downto 0) & d_en_in;

                    time_data <= time_data_reg(size-1);
					a_data <= a_data_reg(size-1);
					a_en <= a_en_reg(size-1);
					b_en <= b_en_reg(size-1);
					c_en <= c_en_reg(size-1);
					d_en <= d_en_reg(size-1);
                    push_done <= '1';
                    pop_done <= '1';
                elsif push = '1' and push_done = '0' and size < 2 then
                    -- perform push
                    time_data_reg <= time_data_reg(time_data_reg'high - 1 downto 0) & time_data_in;
					a_data_reg <= a_data_reg(a_data_reg'high - 1 downto 0) & a_data_in;
					a_en_reg <= a_en_reg(a_en_reg'high - 1 downto 0) & a_en_in;
					b_en_reg <= b_en_reg(b_en_reg'high - 1 downto 0) & b_en_in;
					c_en_reg <= c_en_reg(c_en_reg'high - 1 downto 0) & c_en_in;
					d_en_reg <= d_en_reg(d_en_reg'high - 1 downto 0) & d_en_in;

                    size <= size + 1;
                    av <= '1';
                    is_full <= to_std_logic(size = 1);
                    push_done <= '1';
                elsif pop = '1' and pop_done = '0' and size > 0 then
                    --perform pop
                    time_data <= time_data_reg(size-1);
					a_data <= a_data_reg(size-1);
					a_en <= a_en_reg(size-1);
					b_en <= b_en_reg(size-1);
					c_en <= c_en_reg(size-1);
					d_en <= d_en_reg(size-1);

                    size <= size - 1;
                    is_full <= '0';
                    av <= to_std_logic(size > 1);
                    pop_done <= '1';
                end if;
            else
                if push = '0' then
                    push_done <= '0';
                end if;
                if pop = '0' then 
                    pop_done <= '0';
                end if;
            end if;
        end if;
    end process;

    full <= is_full;
    time_data_out <= time_data;
	a_data_out <= a_data;
	a_en_out <= a_en;
	b_en_out <= b_en;
	c_en_out <= c_en;
	d_en_out <= d_en;
    available <= av;

end behavioral;
