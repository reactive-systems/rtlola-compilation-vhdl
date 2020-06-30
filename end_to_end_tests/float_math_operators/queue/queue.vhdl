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
		a_data_in : in sfixed(8 downto -23);
		a_en_in : in std_logic;
		b_data_in : in sfixed(8 downto -23);
		b_en_in : in std_logic;
		c_data_in : in sfixed(8 downto -23);
		c_en_in : in std_logic;
		plus_op_en_in : in std_logic;
		minus_op_en_in : in std_logic;
		mult_op_en_in : in std_logic;
		func_abs_en_in : in std_logic;
		func_sqrt_en_in : in std_logic;
		counter_en_in : in std_logic;
        full : out std_logic;
        pop : in std_logic;
        time_data_out : out unsigned(63 downto 0);
		a_data_out : out sfixed(8 downto -23);
		a_en_out : out std_logic;
		b_data_out : out sfixed(8 downto -23);
		b_en_out : out std_logic;
		c_data_out : out sfixed(8 downto -23);
		c_en_out : out std_logic;
		plus_op_en_out : out std_logic;
		minus_op_en_out : out std_logic;
		mult_op_en_out : out std_logic;
		func_abs_en_out : out std_logic;
		func_sqrt_en_out : out std_logic;
		counter_en_out : out std_logic;
        available : out std_logic
    );
end queue;

architecture behavioral of queue is

    signal is_full : std_logic;
    signal time_data_reg : unsigned64_array(1 downto 0);
    signal time_data : unsigned(63 downto 0);
	signal a_data_reg : sfixed32_array(1 downto 0);
	signal a_en_reg : bit_array(1 downto 0);
	signal a_data : sfixed(8 downto -23);
	signal a_en: std_logic;
	signal b_data_reg : sfixed32_array(1 downto 0);
	signal b_en_reg : bit_array(1 downto 0);
	signal b_data : sfixed(8 downto -23);
	signal b_en: std_logic;
	signal c_data_reg : sfixed32_array(1 downto 0);
	signal c_en_reg : bit_array(1 downto 0);
	signal c_data : sfixed(8 downto -23);
	signal c_en: std_logic;
	signal plus_op_en_reg : bit_array(1 downto 0);
	signal plus_op_en : std_logic;
	signal minus_op_en_reg : bit_array(1 downto 0);
	signal minus_op_en : std_logic;
	signal mult_op_en_reg : bit_array(1 downto 0);
	signal mult_op_en : std_logic;
	signal func_abs_en_reg : bit_array(1 downto 0);
	signal func_abs_en : std_logic;
	signal func_sqrt_en_reg : bit_array(1 downto 0);
	signal func_sqrt_en : std_logic;
	signal counter_en_reg : bit_array(1 downto 0);
	signal counter_en : std_logic;
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
			b_data_reg(b_data_reg'high downto 0) <= (others => (others => '0'));
			b_en_reg(b_en_reg'high downto 0) <= (others => '0');
			b_data <= (others => '0');
			b_en <= '0';
			c_data_reg(c_data_reg'high downto 0) <= (others => (others => '0'));
			c_en_reg(c_en_reg'high downto 0) <= (others => '0');
			c_data <= (others => '0');
			c_en <= '0';
			plus_op_en_reg <= (others => '0');
			plus_op_en <= '0';
			minus_op_en_reg <= (others => '0');
			minus_op_en <= '0';
			mult_op_en_reg <= (others => '0');
			mult_op_en <= '0';
			func_abs_en_reg <= (others => '0');
			func_abs_en <= '0';
			func_sqrt_en_reg <= (others => '0');
			func_sqrt_en <= '0';
			counter_en_reg <= (others => '0');
			counter_en <= '0';
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
					b_data_reg <= b_data_reg(b_data_reg'high - 1 downto 0) & b_data_in;
					b_en_reg <= b_en_reg(b_en_reg'high - 1 downto 0) & b_en_in;
					c_data_reg <= c_data_reg(c_data_reg'high - 1 downto 0) & c_data_in;
					c_en_reg <= c_en_reg(c_en_reg'high - 1 downto 0) & c_en_in;
					plus_op_en_reg <= plus_op_en_reg(plus_op_en_reg'high - 1 downto 0) & plus_op_en_in;
					minus_op_en_reg <= minus_op_en_reg(minus_op_en_reg'high - 1 downto 0) & minus_op_en_in;
					mult_op_en_reg <= mult_op_en_reg(mult_op_en_reg'high - 1 downto 0) & mult_op_en_in;
					func_abs_en_reg <= func_abs_en_reg(func_abs_en_reg'high - 1 downto 0) & func_abs_en_in;
					func_sqrt_en_reg <= func_sqrt_en_reg(func_sqrt_en_reg'high - 1 downto 0) & func_sqrt_en_in;
					counter_en_reg <= counter_en_reg(counter_en_reg'high - 1 downto 0) & counter_en_in;

                    time_data <= time_data_reg(size-1);
					a_data <= a_data_reg(size-1);
					a_en <= a_en_reg(size-1);
					b_data <= b_data_reg(size-1);
					b_en <= b_en_reg(size-1);
					c_data <= c_data_reg(size-1);
					c_en <= c_en_reg(size-1);
					plus_op_en <= plus_op_en_reg(size-1);
					minus_op_en <= minus_op_en_reg(size-1);
					mult_op_en <= mult_op_en_reg(size-1);
					func_abs_en <= func_abs_en_reg(size-1);
					func_sqrt_en <= func_sqrt_en_reg(size-1);
					counter_en <= counter_en_reg(size-1);
                    push_done <= '1';
                    pop_done <= '1';
                elsif push = '1' and push_done = '0' and size < 2 then
                    -- perform push
                    time_data_reg <= time_data_reg(time_data_reg'high - 1 downto 0) & time_data_in;
					a_data_reg <= a_data_reg(a_data_reg'high - 1 downto 0) & a_data_in;
					a_en_reg <= a_en_reg(a_en_reg'high - 1 downto 0) & a_en_in;
					b_data_reg <= b_data_reg(b_data_reg'high - 1 downto 0) & b_data_in;
					b_en_reg <= b_en_reg(b_en_reg'high - 1 downto 0) & b_en_in;
					c_data_reg <= c_data_reg(c_data_reg'high - 1 downto 0) & c_data_in;
					c_en_reg <= c_en_reg(c_en_reg'high - 1 downto 0) & c_en_in;
					plus_op_en_reg <= plus_op_en_reg(plus_op_en_reg'high - 1 downto 0) & plus_op_en_in;
					minus_op_en_reg <= minus_op_en_reg(minus_op_en_reg'high - 1 downto 0) & minus_op_en_in;
					mult_op_en_reg <= mult_op_en_reg(mult_op_en_reg'high - 1 downto 0) & mult_op_en_in;
					func_abs_en_reg <= func_abs_en_reg(func_abs_en_reg'high - 1 downto 0) & func_abs_en_in;
					func_sqrt_en_reg <= func_sqrt_en_reg(func_sqrt_en_reg'high - 1 downto 0) & func_sqrt_en_in;
					counter_en_reg <= counter_en_reg(counter_en_reg'high - 1 downto 0) & counter_en_in;

                    size <= size + 1;
                    av <= '1';
                    is_full <= to_std_logic(size = 1);
                    push_done <= '1';
                elsif pop = '1' and pop_done = '0' and size > 0 then
                    --perform pop
                    time_data <= time_data_reg(size-1);
					a_data <= a_data_reg(size-1);
					a_en <= a_en_reg(size-1);
					b_data <= b_data_reg(size-1);
					b_en <= b_en_reg(size-1);
					c_data <= c_data_reg(size-1);
					c_en <= c_en_reg(size-1);
					plus_op_en <= plus_op_en_reg(size-1);
					minus_op_en <= minus_op_en_reg(size-1);
					mult_op_en <= mult_op_en_reg(size-1);
					func_abs_en <= func_abs_en_reg(size-1);
					func_sqrt_en <= func_sqrt_en_reg(size-1);
					counter_en <= counter_en_reg(size-1);

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
	b_data_out <= b_data;
	b_en_out <= b_en;
	c_data_out <= c_data;
	c_en_out <= c_en;
	plus_op_en_out <= plus_op_en;
	minus_op_en_out <= minus_op_en;
	mult_op_en_out <= mult_op_en;
	func_abs_en_out <= func_abs_en;
	func_sqrt_en_out <= func_sqrt_en;
	counter_en_out <= counter_en;
    available <= av;

end behavioral;
